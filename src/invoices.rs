use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::*;
use crate::schema::invoices::dsl::*;

pub fn create_invoice(conn: &SqliteConnection, new_invoice: &Invoice) {
    /*let invoice = Invoice {
        invoice_id: "2020-0200".to_string(),
        date: "2020-07-15".to_string(),
        amount_cents: 123456,
        currency: "EUR".to_string(),
        tax_code: "EU16".to_string(),
        order_id: None,
        payment_method: "SEPA".to_string(),
        line_items: "[]".to_string(),
        sales_account: "sales".to_string(),
        customer_account: "customer".to_string(),
        customer_company: None,
        customer_name: "Mhm Hmhm".to_string(),
        customer_address_1: "Fehlerstr. 8".to_string(),
        customer_address_2: None,
        customer_zip: "12161".to_string(),
        customer_city: "Berlin".to_string(),
        customer_state: None,
        customer_country: "DE".to_string(),
        vat_included: "y".to_string(), // FIXME
        replaces_id: None,
        replaced_by_id: None,
        created_at: "2020-07-15 14:05".to_string(),
        updated_at: "2020-07-15 14:05".to_string()
    };*/

    let res = diesel::insert_into(invoices).values(new_invoice).execute(conn);
    println!("create_invoice result: {:?}", res);
}

pub fn get_all_invoices(conn: &SqliteConnection) -> Vec<Invoice> {
    invoices.limit(1000).load::<Invoice>(conn).unwrap()
}

pub fn get_invoice_by_id(conn: &SqliteConnection, id: &String) -> Invoice {
    invoices.find(id).first(conn).unwrap()
}
