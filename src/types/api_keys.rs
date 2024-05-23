pub struct APIKeys {
    pub api_id: i32,
    pub api_hash: String,
}

impl APIKeys {
    pub fn new() -> Self {
        Self {
            api_id: 12345678,
            api_hash: "qwertyuiopasdfghjklzxcvbnm123456".to_string(),
        }
    }
}
