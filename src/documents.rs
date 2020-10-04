use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::*;
use crate::schema::documents::dsl::*;
use diesel::dsl::*;
use uuid::Uuid;
use regex::Regex;

use chrono::prelude::*;
use paperclip::actix::Apiv2Schema;

use crate::util::utc_iso_date_string;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LineItem {
    pub sku: Option<String>,
    pub title: String,
    pub description: String,
    pub quantity: f64,
    pub price_cents: i32,
    pub amount_cents: i32
}

/// Query parameters for Documents
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Apiv2Schema)]
pub struct Query {
    /// Amount in cents
    pub amount: Option<String>,
    pub year: Option<String>,
    pub month: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub text: Option<String>,
    /// Query string for bookings table
    pub bookings_query: Option<String>
}

impl Query {
    pub fn is_empty(&self) -> bool {
        self.amount.is_none() &&
            self.year.is_none() &&
            self.month.is_none() &&
            self.offset.is_none() &&
            self.limit.is_none() &&
            self.text.is_none()
    }
}

pub fn line_items(doc: &Document) -> Vec<LineItem> {
    let mut items:Vec<LineItem> = Vec::new();
    if let Some(items_str) = &doc.line_items {
        if let Ok(i) = serde_json::from_str::<Vec<LineItem>>(items_str) {
            items.extend(i);
        } else {
            println!("warning: could not parse line items {:?}", &items_str);
        }
    }
    items
}

pub fn new_doc_id(conn: &SqliteConnection, doc_kind: &str) -> String {
    let utc: DateTime<Utc> = Utc::now();
    let year = utc.year();
    // FIXME this may assign the same ID to multiple documents of same kind
    // as it only increments up from IDs of docs with a proper  doc_date
    let new_id = match documents.select(max(serial_id))
        .filter(doc_date.like(format!("{}-%", year)))
        .filter(kind.eq(&doc_kind))
        .first::<Option<String>>(conn) {
            Ok(Some(i)) => {
                let parts:Vec<&str> = i.split('-').collect();
                let number = parts.last().unwrap().to_string().parse::<i32>().unwrap();
                format!("{}-{:04}", year, number+1)
            }
            _ => format!("{}-0001", year)
        };

    new_id
}

pub fn create_document(conn: &SqliteConnection, new_document: &Document) -> Document {
    let new_doc_id = new_doc_id(conn, &new_document.kind);
    let doc = Document {
        id: Uuid::new_v4().to_string(),
        serial_id: Some(new_doc_id),
        updated_at: utc_iso_date_string(&Utc::now()), // FIXME missing time?
        created_at: utc_iso_date_string(&Utc::now()),
        ..(*new_document).clone()
    };
    // TODO more input validations
    // TODO improve feedback to user providing bad input
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    assert!(re.is_match(&doc.doc_date));

    let res = diesel::insert_into(documents).values(&doc).execute(conn);
    println!("create_document result: {:?}", res);

    doc
}

pub fn get_documents(conn: &SqliteConnection, q: &Query) -> Vec<Document> {
    let s = documents.into_boxed();

    let s = match q.offset {
        Some(offset) => s.offset(offset),
        _ => s
    };

    let s = match q.limit {
        Some(limit) => s.limit(limit),
        _ => s
    };

    let s = match &q.year {
        Some(year) if year.len()>=4 => s.filter(crate::schema::documents::dsl::doc_date.like(format!("{}-%", year))),
        _ => s
    };

    let s = match &q.month {
        Some(month) if month.len()>=1 => s.filter(crate::schema::documents::dsl::doc_date
                                                  .like(format!("%-{:02}-%", month.parse::<i32>().unwrap()))),
        _ => s
    };

    let s = match &q.text {
        Some(t) if t.len()>=1 => s.filter(foreign_serial_id.like(format!("%{}%", t))
                            .or(customer_account.like(format!("%{}%", t)))
                            .or(account.like(format!("%{}%", t)))
                            .or(order_id.like(format!("%{}%", t)))
                            .or(serial_id.like(format!("%{}%", t)))),
        _ => s
    };

    let s = match &q.amount {
        Some(amt) if amt.len()>=1 => {
            let a = amt.parse::<i32>().unwrap();
            s.filter(amount_cents.gt(a-100)
                     .and(amount_cents.lt(a+100)))
        },
        _ => s
    };

    s.load::<Document>(conn).unwrap()
}

// TODO update document_

pub fn get_document_by_id(conn: &SqliteConnection, uuid: &String) -> Result<Document,diesel::result::Error> {
    documents.find(uuid).first(conn)
}
