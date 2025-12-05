mod core;
mod models;

use core::text::{decode_text, is_mime_type_text};
use models::file::{FileInput, FileMetadata, GroupedFiles};

use chardetng::EncodingDetector;
use std::collections::HashMap;

use napi_derive::napi;

#[napi]
pub fn process_files(files: Vec<FileInput>) -> Vec<GroupedFiles> {
    let mut grouped: HashMap<String, Vec<FileMetadata>> = HashMap::new();

    for file in files {
        let content = file.content.as_ref();
        let size = content.len() as f64;
        let encoding = detect_encoding(content, &file.mime_type);

        let text_content = if is_mime_type_text(&file.mime_type) {
            decode_text(content, &encoding)
        } else {
            String::new()
        };

        let metadata = FileMetadata {
            name: file.filename,
            size,
            processing_time_ms: 0.0,
            encoding,
            text_content,
        };

        grouped
            .entry(file.mime_type.clone())
            .or_insert_with(|| Vec::new())
            .push(metadata);
    }

    grouped
        .into_iter()
        .map(|(mime_type, files)| GroupedFiles { mime_type, files })
        .collect()
}

fn detect_encoding(content: &[u8], mime_type: &str) -> String {
    if is_mime_type_text(mime_type) {
        let mut detector = EncodingDetector::new();
        detector.feed(content, true);
        let encoding = detector.guess(None, true);
        encoding.name().to_string()
    } else {
        "application/octet-stream".to_string()
    }
}
