use actix_web::{error, get, Error, HttpResponse};
use paperclip::actix::{
    api_v2_operation,
    web::{self, Json},
};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use crate::documentimages;
use crate::models::DocumentImage;
use crate::bookingdocs;

#[api_v2_operation]
/// Get DocumentImages
///
/// Returns all DocumentImages that match the specified query.
pub async fn get_documentimages_json(
    pool: web::Data<DbPool>,
    q: web::Query<documentimages::Query>
) -> Result<Json<Vec<DocumentImage>>, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let results = documentimages::get_document_images(&conn, &q);
    Ok(Json(results))
}

#[get("/documentimages")]
pub async fn get_documentimages(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    q: web::Query<documentimages::Query>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let results = documentimages::get_document_images(&conn, &q);

    let mut ctx = tera::Context::new();
    ctx.insert("documentimages", &results);
    ctx.insert("accounts", &bookingdocs::get_all_accounts(&conn));

    let active_di:Option<&DocumentImage> = match &q.active {
        Some(a) => results.iter().find(|di| &di.path == a),
        _ => results.first()
    };
    ctx.insert("active_di", &active_di);

    let mut q = q.into_inner().clone();
    q.active = None;
    let qs = serde_qs::to_string(&q).unwrap();
    ctx.insert("query", &qs);
    ctx.insert("q", &q);

    let s = tmpl.render("documentimages.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {:?}", e)))
        .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
