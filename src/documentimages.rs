extern crate diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::*;
use crate::schema::document_images::dsl::*;
use crate::schema::documents::dsl::*;

#[derive(serde::Deserialize)]
pub struct DocumentImageDocIdInsert {
    pub path: String,
    pub doc_id: String,
}

pub fn get_all_document_images(conn: &SqliteConnection) -> Vec<DocumentImage> {
    document_images.load::<DocumentImage>(conn).unwrap()
}

pub fn set_doc_id(conn: &SqliteConnection, doc_id_insert: &DocumentImageDocIdInsert) -> Result<DocumentImage, diesel::result::Error> {
    let mut doc_img = document_images.find(&doc_id_insert.path).first::<DocumentImage>(conn)?;
    documents.find(&doc_id_insert.doc_id).first::<Document>(conn)?;
    let res = diesel::update(&doc_img).set((doc_id.eq(&doc_id_insert.doc_id), done.eq(true))).execute(conn);
    println!("set_doc_id result: {:?}", res);
    doc_img = document_images.find(&doc_id_insert.path).first(conn).unwrap();
    Ok(doc_img)
}
