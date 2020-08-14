use actix_web::{error, post, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use crate::bookingdocs;

#[post("/bookingdocs")]
pub async fn add_bookingdoc_json(
    pool: web::Data<DbPool>,
    params: web::Json<bookingdocs::BookingDocInsert>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    match bookingdocs::create_bookingdoc(&conn, &params) {
        Ok(booking_doc) => Ok(HttpResponse::Ok().json(&booking_doc)),
        Err(e) => Err(error::ErrorBadRequest(format!("{:?}", e)))

    }
}
