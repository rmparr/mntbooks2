use diesel::prelude::*;

use diesel::sqlite::SqliteConnection;
use crate::models::*;
use crate::schema::booking_docs::dsl::*;
use crate::schema::booking_docs::dsl::id as bookingdoc_id;
use crate::schema::bookings::dsl::*;
use crate::schema::documents::dsl::*;

use diesel::dsl::*;

#[derive(serde::Deserialize)]
pub struct BookingDocInsert {
    pub booking_id: String,
    pub doc_id: String,
}

pub fn create_bookingdoc(conn: &SqliteConnection, new_booking_doc: &BookingDocInsert) -> BookingDoc {

    // ensure booking and document exist; TODO: expressive error instead of panic
    bookings.find(&new_booking_doc.booking_id).get_result::<Booking>(conn).unwrap();
    documents.find(&new_booking_doc.doc_id).get_result::<Document>(conn).unwrap();

    // create incremented ID; TODO: do diesel or sqlite provide facilities for this?
    let new_id = match booking_docs.select(max(bookingdoc_id)).first::<Option<i32>>(conn) {
        Ok(Some(i)) => {
            i + 1 
        }
        _ => 0
    };

    let doc = BookingDoc {
        id: new_id,
        booking_id: Some((&new_booking_doc.booking_id).to_string()),
        doc_id: Some((&new_booking_doc.doc_id).to_string()),
    };

    let res = diesel::insert_into(booking_docs).values(&doc).execute(conn);
    println!("create_bookingdoc result: {:?}", res);

    doc
}

pub fn get_bookingdocs(conn: &SqliteConnection, booking: &Booking) -> Vec<BookingDoc> {
    let results = booking_docs.filter(booking_id.eq(&booking.id)).load::<BookingDoc>(conn).unwrap();
    results
}
