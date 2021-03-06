use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};
use chrono::prelude::*;
use tera::Value;
use std::collections::HashMap;
use rust_decimal::prelude::*;

pub fn utc_iso_date_string(utc: &DateTime<Utc>) -> String {
    format!("{:04}-{:02}-{:02}", utc.year(), utc.month(), utc.day())
}

pub fn fmt_money(cents: &Value, _ctx: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    match cents.as_i64() {
        Some(cents) => Ok(tera::Value::String(Decimal::new(cents,2).to_string())),
        _ => Err(tera::Error::msg("invalid".to_string()))
    }
}

pub fn db_pool_from_url(connspec: &str) -> r2d2::Pool<ConnectionManager<SqliteConnection>> {
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}
