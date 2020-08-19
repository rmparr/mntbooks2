extern crate actix_web;
extern crate actix_files;
extern crate dotenv;
extern crate toml;
#[macro_use]
extern crate diesel;

use actix_web::{error, middleware, web, App, FromRequest, HttpRequest, HttpResponse, HttpServer};
use actix_files::Files;

use tera::Tera;

mod models;
mod schema;
mod bookings;
mod bookingdocs;
mod documents;
mod documentimages;
mod routes;
mod mntconfig;
mod util;

use crate::routes::bookings::*;
use crate::routes::bookings_datev::*;
use crate::routes::documents::*;
use crate::routes::bookingdocs::*;
use crate::routes::documentimages::*;
use crate::models::Document;

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => {
            HttpResponse::UnsupportedMediaType().body(detail)
        }
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let pool = util::db_pool_from_env("DATABASE_URL");

    let bind = "127.0.0.1:8080";

    let mntconfig = mntconfig::Config::new("mntconfig.toml");

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
            .data(mntconfig.clone())
            .wrap(middleware::Logger::default())
            .service(get_bookings)
            .service(get_bookings_datev_csv)
            .service(get_documents)
            .service(get_documents_json)
            .service(get_documentimages_json)
            .service(set_documentimage_docid)
            .service(new_document)
            .service(copy_document)
            .service(get_document)
            .service(add_document)
            .service(add_document_json)
            .service(add_bookingdoc_json)
            .service(Files::new("/css", "static/css/"))
            .service(Files::new("/js", "static/js/"))
            .service(Files::new("/img", "static/img/"))
            .app_data(web::Json::<Document>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))
    })
    .bind(&bind)?
    .run()
    .await
}
