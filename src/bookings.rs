use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::*;
use crate::schema::bookings::dsl::*;

use uuid::Uuid;
use chrono::prelude::*;
use crate::util::utc_iso_date_string;

#[derive(serde::Deserialize,serde::Serialize)]
pub struct Query {
    year: Option<String>,
    month: Option<String>,
    credit_account: Option<String>,
    debit_account: Option<String>,
    details: Option<String>,
    offset: Option<i64>,
    limit: Option<i64>,
    done: Option<String>
}

#[derive(serde::Deserialize)]
pub struct NewBooking {
    pub booking_date: String,
    pub amount_cents: i32,
    pub currency: String,
    pub details: String,
    pub debit_account: String,
    pub credit_account: String,
    pub txn_id: String
}

#[derive(serde::Deserialize)]
pub struct UpdateBooking {
    pub debit_account: String,
    pub credit_account: String,
    pub comment: String,
    pub done: bool,
    pub doc_ids: Option<Vec<String>>,
    pub stay: bool, // stay on the edit booking page?
    pub bookings_query: Option<String> // query string for bookings table
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
        _ => s
    };

    let s = match &q.year {
        Some(year) if year.len()>=4 => s.filter(booking_date.like(format!("{}-%", year))),
        _ => s
    };
    
    let s = match &q.month {
        Some(month) if month.len()>=1 => s.filter(booking_date.like(format!("%-{:02}-%", month.parse::<i32>().unwrap()))),
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
    
    let s = match &q.details {
        Some(d) => s.filter(details.like(format!("%{}%", d))),
        _ => s
    };

    let s = match &q.done {
        Some(d) if d=="true" => s.filter(done.eq(true)),
        Some(d) if d=="false" => s.filter(done.eq(false)),
        _ => s
    };

    s.load::<Booking>(conn).unwrap()
}

pub fn get_booking_by_id(conn: &SqliteConnection, find_id: &String) -> Option<Booking> {
    match bookings.filter(id.eq(find_id)).first::<Booking>(conn) {
        Ok(b) => Some(b),
        _ => None
    }
}

pub fn create_or_update_booking(conn: &SqliteConnection, new_booking: &NewBooking) -> Booking {
    let mut b = Booking {
        id: Uuid::new_v4().to_string(),
        booking_date: new_booking.booking_date.clone(),
        amount_cents: new_booking.amount_cents.clone(),
        currency: new_booking.currency.clone(),
        details: new_booking.details.clone(),
        tax_code: "".to_string(), // FIXME
        debit_account: new_booking.debit_account.clone(),
        credit_account: new_booking.credit_account.clone(),
        txn_id: new_booking.txn_id.clone(),

        created_at: utc_iso_date_string(&Utc::now()),
        updated_at: utc_iso_date_string(&Utc::now()), // FIXME missing time?

        comment: "".to_string(),
        done: false,
    };

    match bookings.filter(txn_id.eq(&b.txn_id)).first::<Booking>(conn) {
        Ok(existing) => {
            b.id = existing.id;
            b.created_at = existing.created_at;
            b.comment = existing.comment;
            b.done = existing.done;
            b.tax_code = existing.tax_code; // FIXME remove
        },
        _ => ()
    }

    diesel::replace_into(bookings).values(&b).execute(conn).unwrap();

    b
}

pub fn update_booking(conn: &SqliteConnection, booking_id: &String, update: &UpdateBooking) {
    diesel::update(bookings.filter(id.eq(&booking_id)))
        .set((credit_account.eq(&update.credit_account),
              debit_account.eq(&update.debit_account),
              comment.eq(&update.comment),
              done.eq(update.done)
        ))
        .execute(conn).unwrap();
}
