
pub struct Secrets {
    pub node_api_key: String,
    pub wallet_password: Option<String>,
}

impl Default for Secrets {
    fn default() -> Self {
        Secrets {
            node_api_key: String::new(), // String vacío como valor por defecto
            wallet_password: None, // None es el valor por defecto para Option
        }
    }
}