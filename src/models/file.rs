pub struct FileInput {
    pub content: Vec<u8>,
    pub mime_type: String,
    pub filename: String,
}

pub struct FileMetadata {
    pub name: String,
    pub size: u64,
    pub processing_time_ms: u64,
    pub encoding: String,
}

pub struct GroupedFiles {
    pub mime_type: String,
    pub files: Vec<FileMetadata>,
}
