use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub version: String, 
    pub path: String,
    pub created_at: SystemTime, 
    pub updated_at: SystemTime
}

impl File {
    pub fn new(path: String) -> Self {
        Self {
            version: String::from("1"), 
            path, 
            created_at: SystemTime::now(),
            updated_at: SystemTime::now()
        }
    }
}