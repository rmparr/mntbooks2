use actix_web::{error, get, post, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::invoices;
use crate::invoices::LineItem;
use crate::models::Invoice;
use crate::mntconfig::Config;

// see: https://github.com/actix/examples/blob/master/diesel/src/main.rs

#[get("/invoices.json")]
pub async fn get_invoices_json(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results = invoices::get_all_invoices(&conn);
    Ok(HttpResponse::Ok().json(results))
}

#[get("/invoices")]
pub async fn get_invoices(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results = invoices::get_all_invoices(&conn);

    let mut ctx = tera::Context::new();
    ctx.insert("invoices", &results);
    
    let s = tmpl.render("invoices.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/invoices/{id}")]
pub async fn get_invoice(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<(String,)>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    
    let result = invoices::get_invoice_by_id(&conn, &path.0);

    let mut ctx = tera::Context::new();
    ctx.insert("invoice", &result);
    ctx.insert("line_items", &invoices::invoice_line_items(&result));
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
pub async fn add_invoice(
    pool: web::Data<DbPool>,
    params: web::Form<Invoice>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let invoice = invoices::create_invoice(&conn, &params);
    Ok(HttpResponse::Ok().json(&invoice))
}

#[post("/invoices.json")]
pub async fn add_invoice_json(
    pool: web::Data<DbPool>,
    params: web::Json<Invoice>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let invoice = invoices::create_invoice(&conn, &params);
    Ok(HttpResponse::Ok().json(&invoice))
}

#[get("/invoices/new")]
pub async fn new_invoice(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();

    let invoice = Invoice {
        doc_id: "".to_string(), // FIXME
        doc_date: "".to_string(), // FIXME today
        kind: "invoice".to_string(),
        amount_cents: 123456,
        currency: "EUR".to_string(),
        tax_code: "EU16".to_string(), // FIXME
        order_id: None,
        payment_method: "SEPA".to_string(),
        line_items: "[]".to_string(), // FIXME 1 empty row
        account: "".to_string(),
        customer_account: "".to_string(),
        customer_company: None,
        customer_name: "".to_string(),
        customer_address_1: "".to_string(),
        customer_address_2: None,
        customer_zip: "".to_string(),
        customer_city: "".to_string(),
        customer_state: None,
        customer_country: "".to_string(),
        vat_included: "true".to_string(), // FIXME
        replaces_id: None,
        replaced_by_id: None,
        created_at: "".to_string(),
        updated_at: "".to_string()
    };

    let items:Vec<LineItem> = vec![LineItem {
        sku: Some("".to_string()),
        title: "".to_string(),
        description: "".to_string(),
        quantity: 1,
        price_cents: 0,
        amount_cents: 0
    }];
    
    ctx.insert("invoice", &invoice);
    ctx.insert("line_items", &items);
    
    let s = tmpl.render("invoice_new.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/invoices/{id}/copy")]
pub async fn copy_invoice(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = invoices::get_invoice_by_id(&conn, &path.0);
    let mut ctx = tera::Context::new();

    ctx.insert("invoice", &result);
    ctx.insert("line_items", &invoices::invoice_line_items(&result));
    
    let s = tmpl.render("invoice_new.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
