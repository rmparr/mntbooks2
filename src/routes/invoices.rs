use actix_web::{error, get, post, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::documents;
use crate::documents::{LineItem, utc_iso_date_string};
use crate::models::Document;
use crate::mntconfig::Config;

use chrono::prelude::*;

// see: https://github.com/actix/examples/blob/master/diesel/src/main.rs

#[get("/invoices.json")]
pub async fn get_documents_json(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results = documents::get_all_documents(&conn);
    Ok(HttpResponse::Ok().json(results))
}

#[get("/invoices")]
pub async fn get_documents(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results = documents::get_all_documents(&conn);

    let mut ctx = tera::Context::new();
    ctx.insert("invoices", &results);
    
    //let s = match tmpl.render("invoices.html", &ctx) {
    //    Ok(x) => x,
    //    Err(e) => { println!("DEBUG {:?}", e); "sorry".to_string() }
    //};
    let s = tmpl.render("invoices.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/invoices/{id}")]
pub async fn get_document(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<(String,)>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    
    let result = documents::get_document_by_id(&conn, &path.0);

    let mut ctx = tera::Context::new();
    ctx.insert("invoice", &result);
    ctx.insert("line_items", &documents::line_items(&result));
    ctx.insert("sender_address", &config.company_address);
    ctx.insert("legal_lines", &config.invoice_legal_lines);
    ctx.insert("bank_lines", &config.invoice_bank_lines);
    ctx.insert("signature_lines", &config.invoice_signature_lines);
    ctx.insert("net_total", &0);
    ctx.insert("tax_rate", &16); // FIXME
    ctx.insert("tax_total", &0);
    ctx.insert("total", &0);
    ctx.insert("outro", &"".to_string());
    ctx.insert("terms", &"".to_string());
    
    let s = tmpl.render("invoice.html", &ctx)
        .map_err(|e| {
            println!("{:?}",e);
            error::ErrorInternalServerError("Template error")
        })
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[post("/invoices")]
pub async fn add_document(
    pool: web::Data<DbPool>,
    params: web::Form<Document>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let document = documents::create_invoice(&conn, &params);
    Ok(HttpResponse::Ok().json(&document))
}

#[post("/invoices.json")]
pub async fn add_document_json(
    pool: web::Data<DbPool>,
    params: web::Json<Document>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let document = documents::create_invoice(&conn, &params);
    Ok(HttpResponse::Ok().json(&document))
}

#[get("/invoices/new")]
pub async fn new_document(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();

    let document= Document {
        id: "_".to_string(), // filled in POST handler
        invoice_id: Some("".to_string()), // filled in POST handler
        quote_id: Some("".to_string()), // filled in POST handler
        doc_date: utc_iso_date_string(&Utc::now()),
        kind: "invoice".to_string(),
        amount_cents: Some(123456),
        currency: Some("EUR".to_string()),
        tax_code: Some("EU16".to_string()), // FIXME
        order_id: None,
        payment_method: Some("SEPA".to_string()),
        line_items: Some("[]".to_string()), // FIXME 1 empty row
        account: Some("".to_string()),
        customer_account: Some("".to_string()),
        customer_company: None,
        customer_name: Some("".to_string()),
        customer_address_1: Some("".to_string()),
        customer_address_2: None,
        customer_zip: Some("".to_string()),
        customer_city: Some("".to_string()),
        customer_state: None,
        customer_country: Some("".to_string()),
        vat_included: Some("true".to_string()), // FIXME
        replaces_id: None,
        replaced_by_id: None,
        created_at: utc_iso_date_string(&Utc::now()),
        updated_at: utc_iso_date_string(&Utc::now())
    };

    let items:Vec<LineItem> = vec![LineItem {
        sku: Some("".to_string()),
        title: "".to_string(),
        description: "".to_string(),
        quantity: 1,
        price_cents: 0,
        amount_cents: 0
    }];
    
    ctx.insert("invoice", &document);
    ctx.insert("line_items", &items);
    
    let s = tmpl.render("invoice_new.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/invoices/{id}/copy")]
pub async fn copy_document(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = documents::get_document_by_id(&conn, &path.0);
    let mut ctx = tera::Context::new();

    let inv = Document {
        invoice_id: Some("".to_string()),
        doc_date: utc_iso_date_string(&Utc::now()),
        ..result.clone()
    };

    ctx.insert("invoice", &inv);
    ctx.insert("line_items", &documents::line_items(&result));
    
    let s = tmpl.render("invoice_new.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
