use std::time::SystemTime;

use grammers_client::types::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMetadata {
    pub version: String,
    pub path: String,
    pub uploaded_at: SystemTime,
    pub creator_id: i64,
    pub creator_access_hash: i64
}

impl FileMetadata {
    pub fn new(path: String, creator: &User) -> Self {
        Self {
            version: String::from("1"),
            path,
            uploaded_at: SystemTime::now(),
            creator_id: creator.id(),
            creator_access_hash: creator.pack().access_hash.unwrap()
        }
    }
}
