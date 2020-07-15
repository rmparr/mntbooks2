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
mod invoices;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// see: https://github.com/actix/examples/blob/master/diesel/src/main.rs

#[get("/invoices")]
async fn get_invoices(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results = invoices::get_all_invoices(&conn);
    Ok(HttpResponse::Ok().json(results))
}

#[post("/invoices")]
async fn add_invoice(
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let invoice = invoices::create_invoice(&conn);
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
