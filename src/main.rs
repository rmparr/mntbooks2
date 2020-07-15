extern crate actix_web;
extern crate dotenv;

#[macro_use]
extern crate diesel;

use diesel::sqlite::SqliteConnection;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use diesel::r2d2::{self, ConnectionManager};

use tera::Tera;

mod models;
mod schema;
mod invoices;
mod routes;

use crate::routes::invoices::*;

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
    // FIXME: why move?
    HttpServer::new(move || {
        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        
        App::new()
            // tera templating
            .data(tera)
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(get_invoices)
            .service(new_invoice)
            .service(add_invoice)
    })
    .bind(&bind)?
    .run()
    .await
}
