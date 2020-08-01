use std::collections::HashMap;

#[derive(Clone, serde::Deserialize)]
pub struct Config {
    pub company_address: Vec<String>,
    pub invoice_legal_lines: Vec<String>,
    pub invoice_bank_lines: Vec<String>,
    pub invoice_signature_lines: Vec<String>,
    pub datev_advisor_id: String,
    pub datev_client_id: String,
    pub datev_account1_map: HashMap<String,String>,
    pub datev_account2_map: HashMap<String,String>
}
