use actix_web::{get, post, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::invoices;

// see: https://github.com/actix/examples/blob/master/diesel/src/main.rs

#[get("/invoices")]
pub async fn get_invoices(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results = invoices::get_all_invoices(&conn);
    Ok(HttpResponse::Ok().json(results))
}

#[post("/invoices")]
pub async fn add_invoice(
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let invoice = invoices::create_invoice(&conn);
    Ok(HttpResponse::Ok().json(invoice))
}
