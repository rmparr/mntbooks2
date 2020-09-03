use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};
use chrono::prelude::*;

pub fn utc_iso_date_string(utc: &DateTime<Utc>) -> String {
    format!("{:04}-{:02}-{:02}", utc.year(), utc.month(), utc.day())
}

pub fn db_pool_from_env(db_var: &str) -> r2d2::Pool<ConnectionManager<SqliteConnection>> {
    let connspec = std::env::var(db_var).expect(db_var);
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}

