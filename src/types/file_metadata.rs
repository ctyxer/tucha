use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMetadata {
    pub version: String,
    pub path: String,
}

impl FileMetadata {
    pub fn new(path: String) -> Self {
        Self {
            version: String::from("1"),
            path
        }
    }
}
