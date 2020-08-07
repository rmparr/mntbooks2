extern crate diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::*;
use crate::schema::document_images::dsl::*;

pub fn get_all_document_images(conn: &SqliteConnection) -> Vec<DocumentImage> {
    document_images.load::<DocumentImage>(conn).unwrap()
}

pub fn create_document_image(conn: &SqliteConnection, img_path: &str) -> DocumentImage {
    // TODO: detect mime, extract text, build PDF and thumbnail etc.
    let doc_img = DocumentImage {
        path: img_path.to_string(),
        pdf_path: "".to_string(),
        mime_type: "".to_string(),
        doc_id: None,
        extracted_text: "".to_string(),
        done: false,
        created_at: "".to_string(),
        updated_at: "".to_string()
    };
    let res = diesel::insert_into(document_images).values(&doc_img).execute(conn);
    println!("create_document_image result: {:?}", res);
    doc_img
}
