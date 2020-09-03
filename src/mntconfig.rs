use std::collections::HashMap;

#[derive(Clone, serde::Deserialize)]
pub struct Config {
    pub company_address: Vec<String>,
    pub invoice_legal_lines: Vec<String>,
    pub invoice_bank_lines: Vec<String>,
    pub invoice_signature_lines: Vec<String>,
    pub docstore_path: String,
    pub datev_export_path: String,
    pub datev_advisor_id: String,
    pub datev_client_id: String,
    pub datev_account1_map: HashMap<String,String>,
    pub datev_account2_map: HashMap<String,String>
}

impl Config {
    pub fn new(toml_path: &str) -> Config {
        let config_str = std::fs::read_to_string(toml_path).unwrap();
        let config: Config = toml::from_str(&config_str).unwrap();
        config
    }
}
