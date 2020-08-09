extern crate diesel;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};

extern crate toml;

extern crate mntbooks;
use mntbooks::documentimages;

fn main() {
    let config_str = std::fs::read_to_string("mntconfig.toml").unwrap();
    let config: mntbooks::mntconfig::Config = toml::from_str(&config_str).unwrap();
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");

    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let conn = pool.get().expect("couldn't get db connection from pool");

    let doc_imgs = documentimages::get_all_document_images(&conn);
    for entry in std::fs::read_dir(config.docstore_path).unwrap() {
        match entry {
            Ok(x) => {
                // FIXME: is_file() may produce unexpected results, check for dotfiles, #-prefixed etc.
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
                        documentimages::create_document_image(&conn, &filename);
                    }
                } else {
                    println!("non-file?: {:?}", x.file_name());
                }
            }
            _ => ()
        }
    }
}
