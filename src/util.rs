use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};
use chrono::prelude::*;
use tera::Value;
use std::collections::HashMap;

pub fn utc_iso_date_string(utc: &DateTime<Utc>) -> String {
    format!("{:04}-{:02}-{:02}", utc.year(), utc.month(), utc.day())
}

pub fn fmt_money(cents: &Value, _ctx: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    match cents.as_i64() {
        Some(cents) => Ok(tera::Value::String(format!("{}.{:02}", cents/100, (cents%100).abs()))),
        _ => Err(tera::Error::msg("invalid".to_string()))
    }
}

pub fn db_pool_from_env(db_var: &str) -> r2d2::Pool<ConnectionManager<SqliteConnection>> {
    let connspec = std::env::var(db_var).expect(db_var);
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}

