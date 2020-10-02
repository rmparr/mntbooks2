extern crate actix_web;
extern crate actix_files;
extern crate dotenv;
extern crate toml;
#[macro_use]
extern crate diesel;

use actix_web::{error, middleware, App, FromRequest, HttpRequest, HttpResponse, HttpServer};
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

use paperclip::actix::{
    OpenApiExt, web::{self, Json},
};

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

    let mntconfig = mntconfig::Config::new("mntconfig.toml");
    
    let bind = mntconfig.http_bind.clone();
    let pool = util::db_pool_from_url(&mntconfig.database_url.clone());

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        let mut tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        tera.register_filter("money", util::fmt_money);
        
        App::new()
            // tera templating
            .data(tera)
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .data(mntconfig.clone())
            .wrap(middleware::Logger::default())
            .wrap_api()
            .service(web::resource("/documents.json")
                     .route(web::get().to(get_documents_json))
                     .route(web::post().to(add_document_json)))
            .service(web::resource("/documentimages.json").route(web::get().to(get_documentimages_json)))
            .service(web::resource("/bookingdocs.json").route(web::post().to(add_bookingdoc_json)))
            .service(web::resource("/bookings.json").route(web::post().to(post_bookings_json)))
            .with_json_spec_at("/api-spec.json")
            .build()
            .service(get_bookings)
            .service(get_booking)
            .service(post_bookings)
            .service(get_bookings_datev_csv)
            .service(get_documents)
            .service(get_documentimages)
            .service(new_document)
            .service(copy_document)
            .service(get_document)
            .service(add_document)
            .service(Files::new("/api-docs", "static/docs/"))
            .service(Files::new("/css", "static/css/"))
            .service(Files::new("/js", "static/js/"))
            .service(Files::new("/img", "static/img/"))
            .service(Files::new("/docstore", &mntconfig.docstore_path).disable_content_disposition())
            .app_data(Json::<Document>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))
    })
    .bind(&bind)?
    .run()
    .await
}
