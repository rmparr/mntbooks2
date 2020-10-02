extern crate toml;
extern crate mntbooks;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::prelude::*;
use mntbooks::models::DocumentImage;
use mntbooks::schema::document_images::dsl::document_images;
use mntbooks::documentimages::create_document_image;
use mntbooks::mntconfig::Config;
use mntbooks::util::{db_pool_from_url,utc_iso_date_string};

pub fn get_all_document_images(conn: &SqliteConnection) -> Vec<DocumentImage> {
    let s = document_images.into_boxed();
    s.load::<DocumentImage>(conn).unwrap()
}

fn main() {
    let config = Config::new("mntconfig.toml");
    let pool = db_pool_from_url(&config.database_url.clone());
    let conn = pool.get().expect("couldn't get db connection from pool");

    let doc_imgs = get_all_document_images(&conn);
    for entry in std::fs::read_dir(config.docstore_path).unwrap() {
        match entry {
            Ok(x) => {
                if x.path().is_file() {
                    let y = x.file_name();
                    let filename = y.into_string().unwrap();
                    let mut is_new = true;
                    for doc_img in &doc_imgs {
                        if doc_img.path == filename {
                            is_new = false;
                            break;
                        }
                    }
                    if is_new {
                        create_document_image(&conn, &filename, None, "".to_string(), false);
                    }
                } else {
                    println!("non-file?: {:?}", x.file_name());
                }
            }
            _ => ()
        }
    }
}
