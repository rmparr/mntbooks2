use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::*;
use crate::schema::bookings::dsl::*;
use diesel::dsl::*;

#[derive(serde::Deserialize)]
pub struct Query {
    year: Option<i32>,
    month: Option<i32>,
    credit_account: Option<String>,
    debit_account: Option<String>,
    offset: Option<i64>,
    limit: Option<i64>
}

pub fn get_all_bookings(conn: &SqliteConnection, q: &Query) -> Vec<Booking> {
    // reason for into_boxed: https://github.com/diesel-rs/diesel/issues/455
    let s = bookings.into_boxed();

    let s = match q.offset {
        Some(offset) => s.offset(offset),
        _ => s
    };
    
    let s = match q.limit {
        Some(limit) => s.limit(limit),
        _ => s.limit(1000)
    };

    let s = match q.year {
        Some(year) => s.filter(booking_date.like(format!("{:04}-%", year))),
        _ => s
    };
    
    let s = match q.month {
        Some(month) => s.filter(booking_date.like(format!("%-{:02}-%", month))),
        _ => s
    };
    
    let s = match &q.credit_account {
        Some(acc) => s.filter(credit_account.like(format!("%{}%", acc))),
        _ => s
    };
    
    let s = match &q.debit_account {
        Some(acc) => s.filter(debit_account.like(format!("%{}%", acc))),
        _ => s
    };

    s.load::<Booking>(conn).unwrap()
}
