use actix_web::{error, post, get, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use crate::documentimages;

#[get("/documentimages.json")]
pub async fn get_documentimages_json(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let results = documentimages::get_all_document_images(&conn);
    Ok(HttpResponse::Ok().json(results))
}

#[post("/documentimages/docid")]
pub async fn set_documentimage_docid(
    pool: web::Data<DbPool>,
    params: web::Json<documentimages::DocumentImageDocIdInsert>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    match documentimages::set_doc_id(&conn, &params) {
        Ok(document_image) =>  Ok(HttpResponse::Ok().json(&document_image)),
        Err(e) => Err(error::ErrorBadRequest(format!("{:?}", e)))
    }
}
