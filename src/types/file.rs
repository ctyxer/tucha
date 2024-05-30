use std::path::PathBuf;

use super::FileMetadata;

#[derive(Debug, Clone)]
pub struct File {
    pub path: PathBuf,
    pub message_id: i32
}

impl File {
    pub fn new(metadata: FileMetadata, message_id: i32) -> Self {
        Self {
            path: metadata.path.into(), 
            message_id,
        }
    }
}