use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::*;
use crate::schema::invoices::dsl::*;
use diesel::dsl::*;

use std::process::Command;
use chrono::prelude::*;

// TODO: missing SKU in frontends
// TODO: document + invoice should have a common interface (trait) so that
// they can be attached to bookings
// like: date, sum, identifier, image (pdf/png/...) url
// TODO: document is missing a kind

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LineItem {
    pub sku: Option<String>,
    pub title: String,
    pub description: String,
    pub quantity: i32,
    pub price_cents: i32,
    pub amount_cents: i32
}

pub fn invoice_line_items(inv: &Invoice) -> Vec<LineItem> {
    let items:Vec<LineItem> = serde_json::from_str(&inv.line_items).unwrap();
    items
}

pub fn invoice_new_id(conn: &SqliteConnection) -> String {
    let utc: DateTime<Utc> = Utc::now();
    let year = utc.year();

    let new_doc_id = match invoices.select(max(doc_id))
        .filter(doc_date.like(format!("{}-%", year)))
        .first::<Option<String>>(conn) {
        Ok(Some(i)) => {
            let parts:Vec<&str> = i.split('-').collect();
            let number = parts.last().unwrap().to_string().parse::<i32>().unwrap();
            format!("{}-{:04}", year, number+1)
        }
        _ => format!("{}-0001", year)
    };

    new_doc_id
}

pub fn create_invoice(conn: &SqliteConnection, new_invoice: &Invoice) -> Invoice {
    // TODO: accept only a subset of data
    // TODO: auto-fill invoice id, updated_at, created_at

    let new_id = invoice_new_id(conn);
    let inv = Invoice {
        doc_id: new_id,
        ..(*new_invoice).clone()
    };
    
    let res = diesel::insert_into(invoices).values(&inv).execute(conn);

    // TODO create pdf
    println!("create_invoice result: {:?}", res);

    inv
}

// TODO update_invoice
// FIXME arbitrary limit
pub fn get_all_invoices(conn: &SqliteConnection) -> Vec<Invoice> {
    invoices.limit(1000).load::<Invoice>(conn).unwrap()
}

pub fn get_invoice_by_id(conn: &SqliteConnection, id: &String) -> Invoice {
    invoices.find(id).first(conn).unwrap()
}
