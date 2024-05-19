use self::metadata::FileMetadata;

pub mod metadata;

#[derive(Debug, Clone)]
pub struct File {
    pub metadata: FileMetadata,
    pub message_id: i32,
    pub name: String
}

impl File {
    pub fn new(metadata: FileMetadata, message_id: i32, name: String) -> Self {
        Self {
            metadata, 
            message_id,
            name
        }
    }
}