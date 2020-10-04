extern crate diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use paperclip::actix::Apiv2Schema;

use crate::models::*;
use crate::schema::document_images::dsl::*;
use crate::schema::documents::dsl::*;

use chrono::prelude::*;
use crate::util::utc_iso_date_string;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Apiv2Schema)]
pub struct Query {
    pub year: Option<String>,
    pub month: Option<String>,
    pub text: Option<String>,
    pub done: Option<String>,
    pub doc_id: Option<String>,
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

    let s = match q.offset {
        Some(offset) => s.offset(offset),
        _ => s
    };

    let s = match q.limit {
        Some(limit) => s.limit(limit),
        _ => s
    };

    let s = match &q.year {
        Some(year) if year.len()>=4 => s.filter(crate::schema::document_images::dsl::created_at.like(format!("{}-%", year))),
        _ => s
    };

    let s = match &q.month {
        Some(month) if month.len()>=1 => s.filter(crate::schema::document_images::dsl::created_at.like(format!("%-{:02}-%", month.parse::<i32>().unwrap()))),
        _ => s
    };

    let s = match &q.text {
        Some(t) => s.filter(extracted_text.like(format!("%{}%", t))
                            .or(path.like(format!("%{}%", t)))),
        _ => s
    };

    let s = match &q.doc_id {
        Some(did) => s.filter(doc_id.eq(did)),
        _ => s
    };

    let s = match &q.done {
        Some(d) if d=="true" => s.filter(done.eq(true)),
        Some(d) if d=="false" => s.filter(done.eq(false)),
        _ => s
    };

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

pub fn create_document_image(conn: &SqliteConnection, img_path: &String, document_id: Option<String>, mime: String, is_done: bool) -> DocumentImage {
    let pdfpath = if mime == "application/pdf" {
        img_path.clone()
    } else {
        "".to_string()
    };

    // TODO: detect mime, extract text, build PDF and thumbnail etc.
    let doc_img = DocumentImage {
        path: img_path.to_string(),
        pdf_path: pdfpath,
        mime_type: mime,
        doc_id: document_id.clone(),
        extracted_text: "".to_string(),
        done: is_done,
        created_at: utc_iso_date_string(&Utc::now()),
        updated_at: utc_iso_date_string(&Utc::now()),
    };
    let res = diesel::insert_into(document_images).values(&doc_img).execute(conn);
    println!("create_document_image result: {:?}", res);
    doc_img
}
