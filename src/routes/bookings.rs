use actix_web::{http, error, get, post, Error, HttpResponse};
use paperclip::actix::{
    api_v2_operation,
    web::{self, Json},
};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::bookings;
use crate::bookingdocs;
use crate::documents;
use crate::models::*;

use base64;
use bytes::Bytes;
use std::collections::HashMap;

#[derive(serde::Serialize)]
struct BookingPlusDocs {
    booking: Booking,
    docs: Vec<Document>
}

#[get("/bookings")]
pub async fn get_bookings(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    q: web::Query<bookings::Query>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results:Vec<Booking> = bookings::get_all_bookings(&conn, &q);
    let mut account_sums:HashMap<String,i32> = HashMap::new();
    
    let mut bookings_plus_docs:Vec<BookingPlusDocs> = Vec::new();
    for booking in results {
        // calculate some stats
        *account_sums.entry(booking.credit_account.clone()).or_insert(0) += booking.amount_cents;
        *account_sums.entry(booking.debit_account.clone()).or_insert(0) -= booking.amount_cents;
        
        let booking_docs = bookingdocs::get_bookingdocs(&conn, &booking);
        let mut docs: Vec<Document> = Vec::new();
        for booking_doc in booking_docs {
            match documents::get_document_by_id(&conn, &booking_doc.doc_id) {
                Ok(doc) => docs.push(doc),
                Err(e) => {
                    println!("get_bookings database inconsistency: doc with id {} linked to booking {} not found!", &booking_doc.doc_id, &booking.id);
                }
            };
        }
        let booking_plus_docs = BookingPlusDocs {
            booking: booking,
            docs: docs,
        };
        bookings_plus_docs.push(booking_plus_docs);
    }

    let query = q.into_inner();
    let mut ctx = tera::Context::new();
    ctx.insert("bookings_plus_docs", &bookings_plus_docs);
    ctx.insert("q", &query);
    ctx.insert("query", &serde_qs::to_string(&query).unwrap());
    ctx.insert("bookings_query", &base64::encode(serde_qs::to_string(&query).unwrap().as_bytes()));
    ctx.insert("account_sums", &account_sums);

    let s = tmpl.render("bookings.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {:?}", e)))
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/bookings/{id}")]
pub async fn get_booking(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
    docq: web::Query<documents::Query>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let booking = bookings::get_booking_by_id(&conn, &path.0).unwrap();
    let mut ctx = tera::Context::new();

    let mut queried_docs = documents::get_documents(&conn, &docq);
    let mut docs:Vec<Document> = vec!();
    
    let doc_ids:Vec<String> = bookingdocs::get_bookingdocs(&conn, &booking).iter().map(|bd| {
        docs.push(documents::get_document_by_id(&conn, &bd.doc_id).unwrap());
        queried_docs.retain(|qd| {
            qd.id != bd.doc_id
        });
        bd.doc_id.clone()
    }).collect();

    docs.append(&mut queried_docs);

    ctx.insert("booking", &booking);
    ctx.insert("doc_ids", &doc_ids);
    ctx.insert("documents", &docs);
    ctx.insert("filter_action", &format!("/bookings/{}", &booking.id));
    ctx.insert("bookings_query", &docq.bookings_query);
    ctx.insert("accounts", &bookingdocs::get_all_accounts(&conn));

    let s = tmpl.render("booking_edit.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {:?}", e)))
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[api_v2_operation]
/// Sync external Booking 
pub async fn post_bookings_json(
    pool: web::Data<DbPool>,
    params: Json<bookings::NewBooking>
) -> Result<Json<Booking>, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // TODO: validate input

    // booking_date: YYYY-MM-DD HH:MM:SS
    // currency: ABC (whitelist)
    // accounts: [a-z:]+

    let b = bookings::sync_external_booking(&conn, &params);
    
    Ok(Json(b))
}

#[post("/bookings/{id}")]
pub async fn post_bookings(
    pool: web::Data<DbPool>,
    bytes: Bytes,
    path: web::Path<(String,)>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // TODO: validate input

    // booking_date: YYYY-MM-DD HH:MM:SS
    // currency: ABC (whitelist)
    // accounts: [a-z:]+

    let booking_id = &path.0;
    let text = String::from_utf8(bytes.to_vec()).unwrap();
    
    let qs = serde_qs::Config::new(1, false); // max_depth, strict
    match qs.deserialize_str(&text) {
        Ok(params) => {
            bookings::update_booking(&conn, booking_id, &params);
            
            // update the booking -> document associations

            bookingdocs::delete_all_bookingdocs_for_booking(&conn, booking_id);
            match &params.doc_ids {
                Some(doc_ids) => {
                    for di in doc_ids {
                        let bdi = &bookingdocs::BookingDocInsert {
                            booking_id: booking_id.clone(),
                            doc_id: di.clone()
                        };
                        bookingdocs::create_bookingdoc(&conn, bdi).unwrap();
                    }
                },
                _ => ()
            }
            
            let href = if params.stay {
                match params.bookings_query {
                    Some(q) => format!("/bookings/{}?bookings_query={}", booking_id, &q),
                    _ => format!("/bookings/{}", booking_id)
                }
            } else {
                let query_string = match &params.bookings_query {
                    Some(s) if s.len()>0 => String::from_utf8(base64::decode(s).unwrap()).unwrap(),
                    _ => "".to_string()
                };
                "/bookings?".to_string()+&query_string
            };

            Ok(HttpResponse::Found().header(http::header::LOCATION, href).finish())
        },
        Err(e) => Err(error::ErrorBadRequest(format!("{:?}", e)))
    }
}
