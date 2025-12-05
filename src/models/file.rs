use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

#[napi(object)]
pub struct FileInput {
    pub content: Buffer,
    pub mime_type: String,
    pub filename: String,
}

#[napi(object)]
pub struct FileMetadata {
    pub name: String,
    pub size: f64,
    pub processing_time_ms: f64,
    pub encoding: String,
    pub text_content: String,
}

#[napi(object)]
pub struct GroupedFiles {
    pub mime_type: String,
    pub files: Vec<FileMetadata>,
}
