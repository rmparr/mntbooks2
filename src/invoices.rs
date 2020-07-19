use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::*;
use crate::schema::invoices::dsl::*;

use std::process::Command;

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

pub fn create_invoice(conn: &SqliteConnection, new_invoice: &Invoice) {
    // TODO: accept only a subset of data
    // TODO: auto-fill invoice id, updated_at, created_at
    
    let res = diesel::insert_into(invoices).values(new_invoice).execute(conn);

    // TODO create pdf
    println!("create_invoice result: {:?}", res);
}

// TODO update_invoice

pub fn get_all_invoices(conn: &SqliteConnection) -> Vec<Invoice> {
    invoices.limit(1000).load::<Invoice>(conn).unwrap()
}

pub fn get_invoice_by_id(conn: &SqliteConnection, id: &String) -> Invoice {
    invoices.find(id).first(conn).unwrap()
}
