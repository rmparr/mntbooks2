#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

mod schema;

#[derive(Queryable)]
struct Invoice {
    id: String,
    invoice_date: String,
    amount: i32,
    line_items: String
}

fn connect_db() -> SqliteConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

fn main() {
    use schema::invoices::dsl::*;

    println!("Hello, world!");
    let db = connect_db();

    let results = invoices.limit(5).load::<Invoice>(&db).expect("Error loading posts");

    for invoice in results {
        println!("Invoice: {} {} {}", invoice.id, invoice.invoice_date, invoice.amount);
    }
}
