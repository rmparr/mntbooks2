use actix_web::{http, error, get, post, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::bookings;
use crate::bookingdocs;
use crate::documents;
use crate::models::*;

use bytes::Bytes;

#[derive(serde::Serialize)]
struct BookingPlusDocs {
    booking: Booking,
    docs: Vec<String>
}

#[get("/bookings")]
pub async fn get_bookings(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    q: web::Query<bookings::Query>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results:Vec<Booking> = bookings::get_all_bookings(&conn, &q);

    let mut bookings_plus_docs:Vec<BookingPlusDocs> = Vec::new();
    for booking in results {
        let booking_docs = bookingdocs::get_bookingdocs(&conn, &booking);
        let mut doc_ids: Vec<String> = Vec::new();
        for booking_doc in booking_docs {
            doc_ids.push(booking_doc.doc_id);
        }
        let booking_plus_docs = BookingPlusDocs {
            booking: booking,
            docs: doc_ids,
        };
        bookings_plus_docs.push(booking_plus_docs);
    }

    let mut ctx = tera::Context::new();
    ctx.insert("bookings_plus_docs", &bookings_plus_docs);
    
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
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let booking = bookings::get_booking_by_id(&conn, &path.0).unwrap();
    let mut ctx = tera::Context::new();
    
    let doc_ids:Vec<String> = bookingdocs::get_bookingdocs(&conn, &booking).iter().map(|bd| {
        bd.doc_id.clone()
    }).collect();

    let docs = documents::get_all_documents(&conn);

    ctx.insert("booking", &booking);
    ctx.insert("doc_ids", &doc_ids);
    ctx.insert("documents", &docs);
    
    let s = tmpl.render("booking_edit.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {:?}", e)))
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[post("/bookings.json")]
pub async fn post_bookings_json(
    pool: web::Data<DbPool>,
    params: web::Json<bookings::NewBooking>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // TODO: validate input

    // booking_date: YYYY-MM-DD HH:MM:SS
    // currency: ABC (whitelist)
    // accounts: [a-z:]+

    let b = bookings::create_or_update_booking(&conn, &params);
    
    Ok(HttpResponse::Ok().content_type("application/json").json(b))
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
                        bookingdocs::create_bookingdoc(&conn, bdi);
                    }
                },
                _ => ()
            }
            
            let href = "/bookings"; // TODO: ?".to_string() + &qs;
            Ok(HttpResponse::Found().header(http::header::LOCATION, href).finish())
        },
        Err(e) => Err(error::ErrorBadRequest(format!("{:?}", e)))
    }
}
