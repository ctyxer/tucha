use super::{FileMetadata, Path};

#[derive(Debug, Clone)]
pub struct File {
    pub path: Path,
    pub message_id: i32
}

impl File {
    pub fn new(metadata: FileMetadata, message_id: i32) -> Self {
        Self {
            path: Path::from(metadata.path), 
            message_id,
        }
    }
}