use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMetadata {
    pub path: String,
}

impl FileMetadata {
    pub fn new(path: String) -> Self {
        Self {
            path
        }
    }
}
