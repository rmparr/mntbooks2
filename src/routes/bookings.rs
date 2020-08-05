use actix_web::{error, get, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::bookings;
use crate::bookingdocs;
use crate::models::*;

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

    let mut bookings_plus_docs:Vec<BookingPlusDocs> =Vec::new();
    for booking in results {
        let booking_docs = bookingdocs::get_bookingdocs(&conn, &booking);
        let mut doc_ids: Vec<String> = Vec::new();
        for booking_doc in booking_docs {
            doc_ids.extend(booking_doc.doc_id);
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
