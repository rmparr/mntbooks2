extern crate toml;
extern crate mntbooks;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::prelude::*;
use mntbooks::models::DocumentImage;
use mntbooks::schema::document_images::dsl::document_images;
use mntbooks::mntconfig::Config;
use mntbooks::util::{db_pool_from_env,utc_iso_date_string};

fn create_document_image(conn: &SqliteConnection, img_path: &str) -> DocumentImage {
    // TODO: detect mime, extract text, build PDF and thumbnail etc.
    let doc_img = DocumentImage {
        path: img_path.to_string(),
        pdf_path: "".to_string(),
        mime_type: "".to_string(),
        doc_id: None,
        extracted_text: "".to_string(),
        done: false,
        created_at: utc_iso_date_string(&Utc::now()),
        updated_at: utc_iso_date_string(&Utc::now()),
    };
    let res = diesel::insert_into(document_images).values(&doc_img).execute(conn);
    println!("create_document_image result: {:?}", res);
    doc_img
}

pub fn get_all_document_images(conn: &SqliteConnection) -> Vec<DocumentImage> {
    let s = document_images.into_boxed();
    s.load::<DocumentImage>(conn).unwrap()
}

fn main() {
    let config = Config::new("mntconfig.toml");
    let pool = db_pool_from_env("DATABASE_URL");
    let conn = pool.get().expect("couldn't get db connection from pool");

    let doc_imgs = get_all_document_images(&conn);
    for entry in std::fs::read_dir(config.docstore_path).unwrap() {
        match entry {
            Ok(x) => {
                if x.path().is_file() {
                    let y = x.file_name();
                    let filename = y.to_str().unwrap();
                    let mut is_new = true;
                    for doc_img in &doc_imgs {
                        if &(doc_img.path.as_str()) == &filename {
                            is_new = false;
                            break;
                        }
                    }
                    if is_new {
                        create_document_image(&conn, &filename);
                    }
                } else {
                    println!("non-file?: {:?}", x.file_name());
                }
            }
            _ => ()
        }
    }
}
