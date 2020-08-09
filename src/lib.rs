#[macro_use]
extern crate diesel;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};

mod schema;

mod models;

pub mod documentimages;
pub mod mntconfig;

pub fn db_pool_from_env(db_var: &str) -> r2d2::Pool<ConnectionManager<SqliteConnection>> {
    let connspec = std::env::var(db_var).expect(db_var);
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}

// TODO: go through this lib even for main.rs
