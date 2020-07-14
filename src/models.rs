use crate::schema::*;

#[derive(Queryable, Insertable, serde::Serialize)]
pub struct Invoice {
    pub invoice_id: String,
    pub date: String,
    pub amount_cents: i32,
    pub currency: String,
    pub tax_code: String,
    pub order_id: Option<String>,
    pub payment_method: String,
    pub line_items: String,
    pub sales_account: String,
    pub customer_account: String,
    pub customer_company: Option<String>,
    pub customer_name: String,
    pub customer_address_1: String,
    pub customer_address_2: Option<String>,
    pub customer_zip: String,
    pub customer_city: String,
    pub customer_state: Option<String>,
    pub customer_country: String,
    pub vat_included: String,
    pub replaces_id: String,
    pub replaced_by_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>
}
