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

#[napi(object)]
pub struct SimilarityMatch {
    pub reference_index: u32,
    pub similarity_percentage: f64,
}

#[napi(object)]
pub struct FileMetadataWithSimilarity {
    pub name: String,
    pub size: f64,
    pub processing_time_ms: f64,
    pub encoding: String,
    pub text_content: String,
    pub similarity_matches: Vec<SimilarityMatch>,
}

#[napi(object)]
pub struct GroupedFilesWithSimilarity {
    pub mime_type: String,
    pub files: Vec<FileMetadataWithSimilarity>,
}
