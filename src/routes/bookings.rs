use actix_web::{error, get, post, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::bookings;
use crate::models::Booking;

#[get("/bookings")]
pub async fn get_bookings(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    q: web::Query<bookings::Query>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results:Vec<Booking> = bookings::get_all_bookings(&conn, &q);

    let mut ctx = tera::Context::new();
    ctx.insert("bookings", &results);
    
    let s = tmpl.render("bookings.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
