use actix_web::{get, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use mntbooks::documentimages;

#[get("/documentimages.json")]
pub async fn get_documentimages_json(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let results = documentimages::get_all_document_images(&conn);
    Ok(HttpResponse::Ok().json(results))
}

