use actix_web::{error, get, post, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::invoices;
use crate::models::Invoice;

// see: https://github.com/actix/examples/blob/master/diesel/src/main.rs

#[get("/invoices.json")]
pub async fn get_invoices_json(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results = invoices::get_all_invoices(&conn);
    Ok(HttpResponse::Ok().json(results))
}

#[get("/invoices")]
pub async fn get_invoices(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results = invoices::get_all_invoices(&conn);

    let mut ctx = tera::Context::new();
    ctx.insert("invoices", &results);
    
    let s = tmpl.render("invoices.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/invoices/{id}")]
pub async fn get_invoice(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let result = invoices::get_invoice_by_id(&conn, &path.0);

    let line_items:Vec<String> = vec![];

    // TODO load from config
    let sender_address:Vec<String> = vec!["ACME, Inc.".to_string()];
    let legal_lines:Vec<String> = vec![];
    let bank_lines:Vec<String> = vec![];
    
    let mut ctx = tera::Context::new();
    ctx.insert("invoice", &result);
    ctx.insert("line_items", &line_items);
    ctx.insert("sender_address", &sender_address);
    ctx.insert("legal_lines", &legal_lines);
    ctx.insert("bank_lines", &bank_lines);
    ctx.insert("net_total", &0);
    ctx.insert("tax_rate", &16);
    ctx.insert("tax_total", &0);
    ctx.insert("total", &0);
    ctx.insert("outro", &"".to_string());
    ctx.insert("terms", &"".to_string());
    
    let s = tmpl.render("invoice.html", &ctx)
        .map_err(|e| {
            println!("{:?}",e);
            error::ErrorInternalServerError("Template error")
        })
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
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

#[post("/invoices.json")]
pub async fn add_invoice_json(
    pool: web::Data<DbPool>,
    params: web::Json<Invoice>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    invoices::create_invoice(&conn, &params);
    Ok(HttpResponse::Ok().json(&*params))
}

#[get("/invoices/new")]
pub async fn new_invoice(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {
    let ctx = tera::Context::new();
    
    let s = tmpl.render("invoice_new.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
