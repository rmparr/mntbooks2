extern crate diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::*;
use crate::schema::document_images::dsl::*;
use crate::schema::documents::dsl::*;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Query {
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub text: Option<String>,
    pub done: Option<bool>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub active: Option<String>
}

#[derive(serde::Deserialize)]
pub struct DocumentImageUpdate {
    pub path: String,
    pub doc_id: String,
    pub done: bool
}

pub fn get_document_images(conn: &SqliteConnection, q: &Query) -> Vec<DocumentImage> {
    let s = document_images.into_boxed();

    println!("query: {:?}", &q);

    let s = match q.offset {
        Some(offset) => s.offset(offset),
        _ => s
    };

    let s = match q.limit {
        Some(limit) => s.limit(limit),
        _ => s
    };

    let s = match q.year {
        Some(year) => s.filter(crate::schema::document_images::dsl::created_at.like(format!("{:04}-%", year))),
        _ => s
    };

    let s = match q.month {
        Some(month) => s.filter(crate::schema::document_images::dsl::created_at.like(format!("%-{:02}-%", month))),
        _ => s
    };

    let s = match &q.text {
        Some(t) => s.filter(extracted_text.like(format!("%{}%", t))
                            .or(path.like(format!("%{}%", t)))),
        _ => s
    };

    let s = match q.done {
        Some(d) => s.filter(done.eq(d)),
        _ => s
    };

    s.load::<DocumentImage>(conn).unwrap()
}

pub fn get_all_document_images(conn: &SqliteConnection) -> Vec<DocumentImage> {
    let s = document_images.into_boxed();
    s.load::<DocumentImage>(conn).unwrap()
}

pub fn update_document_image(conn: &SqliteConnection, di_update: &DocumentImageUpdate) -> Result<DocumentImage, diesel::result::Error> {
    let mut doc_img = document_images.find(&di_update.path).first::<DocumentImage>(conn)?;
    documents.find(&di_update.doc_id).first::<Document>(conn)?;
    let res = diesel::update(&doc_img).set((doc_id.eq(&di_update.doc_id), done.eq(true))).execute(conn);
    println!("set_doc_id result: {:?}", res);
    doc_img = document_images.find(&di_update.path).first(conn).unwrap();
    Ok(doc_img)
}
