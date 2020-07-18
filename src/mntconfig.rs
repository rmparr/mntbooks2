#[derive(Clone, serde::Deserialize)]
pub struct Config {
    pub company_address: Vec<String>,
    pub invoice_legal_lines: Vec<String>,
    pub invoice_bank_lines: Vec<String>,
}
