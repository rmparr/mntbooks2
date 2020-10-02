use actix_web::{error, Error};
use paperclip::actix::{
    api_v2_operation,
    web::{self, Json},
};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use crate::bookingdocs;
use crate::models;

#[api_v2_operation]
/// Create a Booking↔Doc Link
pub async fn add_bookingdoc_json(
    pool: web::Data<DbPool>,
    params: Json<bookingdocs::BookingDocInsert>
) -> Result<Json<models::BookingDoc>, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    match bookingdocs::create_bookingdoc(&conn, &params) {
        Ok(booking_doc) => Ok(Json(booking_doc)),
        Err(e) => Err(error::ErrorBadRequest(format!("{:?}", e)))
    }
}
