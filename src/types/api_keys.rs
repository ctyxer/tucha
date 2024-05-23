pub struct APIKeys {
    pub api_id: i32,
    pub api_hash: String,
}

impl APIKeys {
    pub fn new() -> Self {
        Self {
            api_id: 12345678, // insert there your api_id from https://my.telegram.org/apps
            api_hash: "qwerty".to_string() // insert there your api_hash from https://my.telegram.org/apps
        }
    }
}
