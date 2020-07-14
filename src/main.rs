extern crate actix_web;
extern crate dotenv;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use diesel::r2d2::{self, ConnectionManager};

mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// https://github.com/actix/examples/blob/master/diesel/src/main.rs

#[get("/invoices")]
async fn get_invoices(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    use crate::schema::invoices::dsl::*;
    let conn = pool.get().expect("couldn't get db connection from pool");

    let invoice = invoices.first::<models::Invoice>(&conn).unwrap();

    Ok(HttpResponse::Ok().json(invoice))
}

#[post("/invoices")]
async fn add_invoice(
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    use crate::schema::invoices::dsl::*;
    let conn = pool.get().expect("couldn't get db connection from pool");

    let invoice = models::Invoice {
        invoice_id: "2020-0200".to_string(),
        date: "2020-07-15".to_string(),
        amount_cents: 123456,
        currency: "EUR".to_string(),
        tax_code: "EU16".to_string(),
        order_id: None,
        payment_method: "SEPA".to_string(),
        line_items: "[]".to_string(),
        sales_account: "sales".to_string(),
        customer_account: "customer".to_string(),
        customer_company: None,
        customer_name: "Mhm Hmhm".to_string(),
        customer_address_1: "Fehlerstr. 8".to_string(),
        customer_address_2: None,
        customer_zip: "12161".to_string(),
        customer_city: "Berlin".to_string(),
        customer_state: None,
        customer_country: "DE".to_string(),
        vat_included: "y".to_string(), // FIXME
        replaces_id: "".to_string(), // FIXME should be option
        replaced_by_id: None,
        created_at: None, // FIXME shouldn't be option
        updated_at: None
    };

    diesel::insert_into(invoices).values(&invoice).execute(&conn).unwrap();
    
    Ok(HttpResponse::Ok().json(invoice))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(get_invoices)
            .service(add_invoice)
    })
    .bind(&bind)?
    .run()
    .await
}
