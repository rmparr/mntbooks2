use actix_web::{error, get, post, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::invoices;
use crate::models::Invoice;

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
    pool: web::Data<DbPool>,
    params: web::Form<Invoice>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    invoices::create_invoice(&conn, &params);
    Ok(HttpResponse::Ok().json(&*params))
}

#[get("/invoices/new")]
pub async fn new_invoice(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {

    let mut ctx = tera::Context::new();
    //ctx.insert("name", &name.to_owned());
    
    let s = tmpl.render("invoice_new.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
