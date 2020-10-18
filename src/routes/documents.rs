use actix_web::{http, error, get, post, Error, HttpResponse};
use paperclip::actix::{
    api_v2_operation,
    web::{self, Json},
};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use std::path::Path;
use std::io::Write;
use std::fs::File;
use std::process::Command;

use crate::documents;
use crate::documents::LineItem;
use crate::documentimages::*;
use crate::models::Document;
use crate::models::DocumentImage;
use crate::mntconfig::Config;

use chrono::prelude::*;
use crate::util::utc_iso_date_string;
use rust_decimal::prelude::*;

#[derive(serde::Deserialize)]
pub struct DocumentImageForm {
    // for Document:
    pub doc_date: String,
    pub amount_cents: Option<i32>,
    pub currency: Option<String>,
    pub foreign_serial_id: Option<String>,
    pub customer_account: Option<String>,
    // for DocumentImage:
    pub path: String,
    pub done: bool
}

#[api_v2_operation]
/// Get Documents
///
/// Returns all Document objects matching the given query.
pub async fn get_documents_json(
    pool: web::Data<DbPool>,
    q: web::Query<documents::Query>,
) -> Result<Json<Vec<Document>>, ()> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results = documents::get_documents(&conn, &q);
    Ok(Json(results))
}

#[get("/documents")]
pub async fn get_documents(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    q: web::Query<documents::Query>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let mut query = q.into_inner();
    if query.is_empty() {
        let now = &Utc::now();
        query.year = Some(now.year().to_string());
        query.month = Some(now.month().to_string());
    }

    let results = documents::get_documents(&conn, &query);

    let mut ctx = tera::Context::new();
    ctx.insert("documents", &results);
    ctx.insert("q", &query);

    let s = tmpl.render("documents.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {:?}", e)))
        .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub fn document_to_html(config: &Config, tmpl: &tera::Tera, doc: &Document) -> String {
    let payment_method = &doc.payment_method.clone().unwrap();
    let empty_payment_terms = "".to_string();
    let payment_terms = match config.invoice_payment_terms.get(payment_method) {
        Some(terms) => terms,
        None => {
            println!("warning: unknown payment method {:?} in document {:?}", payment_method, doc.serial_id);
           &empty_payment_terms
        }
    };

    let mut tax_rate = doc.get_tax_rate(config);
    let mut total = doc.get_total();
    let mut net_total = doc.get_net_total(config);
    let mut tax_total = doc.get_tax_total(config);
    net_total.rescale(2);
    tax_total.rescale(2);
    total.rescale(2);
    tax_rate *= Decimal::new(100,0);
    tax_rate.rescale(1);
    let mut outro = "".to_string();
    if tax_rate == Decimal::new(0,0) {
        outro += &config.invoice_outro_no_tax;
    }

    let mut ctx = tera::Context::new();
    ctx.insert("document", doc);
    ctx.insert("line_items", &documents::line_items(doc));
    ctx.insert("sender_address", &config.company_address);
    ctx.insert("legal_lines", &config.invoice_legal_lines);
    ctx.insert("bank_lines", &config.invoice_bank_lines);
    ctx.insert("signature_lines", &config.invoice_signature_lines);
    ctx.insert("net_total", &net_total);
    ctx.insert("tax_rate", &tax_rate);
    ctx.insert("tax_total", &tax_total);
    ctx.insert("total", &total);
    ctx.insert("outro", &outro);
    ctx.insert("terms", &payment_terms);

    let html = tmpl.render("document.html", &ctx)
        .map_err(|e| {
            println!("{:?}",e);
            error::ErrorInternalServerError("Template error")
        })
        .unwrap();
    html
}

#[get("/documents/{id}")]
pub async fn get_document(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<(String,)>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let result = match documents::get_document_by_id(&conn, &path.0) {
        Ok(res) => res,
        Err(e) => {
            assert_eq!(diesel::NotFound, e);
            return Err(error::ErrorNotFound(e))
        }
    };
    let html = document_to_html(&config, &tmpl, &result);

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[post("/documents")]
/// Create a Document for an existing DocumentImage via form POST
pub async fn add_document(
    pool: web::Data<DbPool>,
    dif: web::Form<DocumentImageForm>,
    q: web::Query<Query>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // TODO: enforce doc_date format

    let d = Document {
        kind: "receipt".to_string(),
        doc_date: dif.doc_date.clone(),
        amount_cents: dif.amount_cents,
        currency: dif.currency.clone(),
        foreign_serial_id: dif.foreign_serial_id.clone(),
        customer_account: dif.customer_account.clone(),
        updated_at: "".to_string(),
        created_at: "".to_string(),
        ..Default::default()
    };

    let document = documents::create_document(&conn, &d);

    match update_document_image(&conn, &DocumentImageUpdate {
        path: dif.path.clone(),
        doc_id: document.id.clone(),
        done: dif.done
    }) {
        Ok(_document_image) => {
            let q = q.into_inner();
            let qs = serde_qs::to_string(&q).unwrap();
            let href = "/documentimages?".to_string() + &qs;
            Ok(HttpResponse::Found().header(http::header::LOCATION, href).finish())
        },
        Err(e) => Err(error::ErrorBadRequest(format!("{:?}", e)))
    }

}

// TODO Error handling
pub fn create_pdf_document_image(config: &Config, conn: &SqliteConnection, tmpl: &tera::Tera, doc: &Document, pdf_path: &String) -> Result<DocumentImage, Error> {
    let pdf_docstore_path = Path::new(&config.docstore_path.clone())
        .join(pdf_path);

    let html = document_to_html(config, tmpl, doc);

    let temp_html_path = format!("{}.html",pdf_path);
    let mut temp_file = File::create(&temp_html_path).unwrap();
    temp_file.write_all(html.as_bytes()).unwrap();

    let mut wkpdf = Command::new("wkhtmltopdf");
    wkpdf.arg("--page-size");
    wkpdf.arg("A4");
    wkpdf.arg("--margin-left");
    wkpdf.arg("20mm");
    wkpdf.arg("--margin-right");
    wkpdf.arg("20mm");
    wkpdf.arg("--margin-top");
    wkpdf.arg("20mm");
    wkpdf.arg("--margin-bottom");
    wkpdf.arg("20mm");
    wkpdf.arg(&temp_html_path);
    wkpdf.arg(&pdf_docstore_path);
    let result = wkpdf.output();
    println!("wkhtmltopdf {:?} -> {:?} -> {:?}", &temp_html_path, &pdf_docstore_path, result);

    std::fs::remove_file(temp_html_path).unwrap();

    Ok(create_document_image(conn, pdf_path, Some(doc.id.clone()), "application/pdf".to_string(), true))
}

#[api_v2_operation]
/// Create a Document
///
/// Creates a new Document such as an Invoice, Quote, or Refund.
/// As a side effect, creates a PDF version as a linked DocumentImage
pub async fn add_document_json(
    pool: web::Data<DbPool>,
    tmpl: web::Data<tera::Tera>,
    config: web::Data<Config>,
    params: Json<Document>
) -> Result<Json<Document>,()> {
    // TODO: check if SEPA or sepa

    let conn = pool.get().expect("couldn't get db connection from pool");

    let doc = documents::create_document(&conn, &params);

    // create pdf
    // 1. create html
    // 2. create pdf from html and save it
    // 3. create a documentimage with the pdf path

    let pdf_path = format!("{}-{}-{}.pdf", &doc.kind, &doc.order_id.clone().unwrap(), &doc.serial_id.clone().unwrap());

    create_pdf_document_image(&config, &conn, &tmpl, &doc, &pdf_path).unwrap();

    Ok(Json(doc))
}

#[get("/documents/new")]
pub async fn new_document(
    tmpl: web::Data<tera::Tera>,
    config: web::Data<Config>
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();

    let document = Document {
        id: "_".to_string(), // filled in POST handler
        serial_id: Some("".to_string()), // filled in POST handler
        foreign_serial_id: Some("".to_string()), // filled in POST handler
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
        // FIXME: why set these here if the values are overwritten by create_document?
        created_at: utc_iso_date_string(&Utc::now()),
        updated_at: utc_iso_date_string(&Utc::now())
    };

    let items:Vec<LineItem> = vec![LineItem {
        sku: Some("".to_string()),
        title: "".to_string(),
        description: "".to_string(),
        quantity: 1.0,
        price_cents: 0,
        amount_cents: 0
    }];

    let mut invoice_payment_terms: Vec<String> = config.invoice_payment_terms.iter()
        .map(|kv| kv.0.to_string())
        .collect();

    invoice_payment_terms.sort();

    let mut tax_rates: Vec<String> = config.tax_rates.iter()
        .map(|kv| kv.0.to_string())
        .collect();

    tax_rates.sort();

    ctx.insert("document", &document);
    ctx.insert("line_items", &items);
    ctx.insert("invoice_payment_terms", &invoice_payment_terms);
    ctx.insert("tax_rates", &tax_rates);

    let s = tmpl.render("document_new.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {:?}", e)))
        .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/documents/{id}/copy")]
pub async fn copy_document(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
    config: web::Data<Config>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let result = match documents::get_document_by_id(&conn, &path.0) {
        Ok(res) => res,
        Err(e) => {
            assert_eq!(diesel::NotFound, e);
            return Err(error::ErrorNotFound(e))
        }
    };
    let mut ctx = tera::Context::new();

    let doc = Document {
        serial_id: Some("".to_string()),
        doc_date: utc_iso_date_string(&Utc::now()),
        ..result.clone()
    };

    let mut invoice_payment_terms: Vec<String> = config.invoice_payment_terms.iter()
        .map(|kv| kv.0.to_string())
        .collect();

    invoice_payment_terms.sort();

    let mut tax_rates: Vec<String> = config.tax_rates.iter()
        .map(|kv| kv.0.to_string())
        .collect();

    tax_rates.sort();

    ctx.insert("document", &doc);
    ctx.insert("line_items", &documents::line_items(&result));
    ctx.insert("invoice_payment_terms", &invoice_payment_terms);
    ctx.insert("tax_rates", &tax_rates);

    // TODO Make .kind selected in template somehow?

    let s = tmpl.render("document_new.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {:?}", e)))
        .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
