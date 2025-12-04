mod models;
use models::file::{FileInput, FileMetadata, GroupedFiles};

use chardetng::EncodingDetector;
use std::collections::HashMap;

pub fn process_files(files: Vec<FileInput>) -> Vec<GroupedFiles> {
    let mut grouped: HashMap<String, Vec<FileMetadata>> = HashMap::new();

    for file in files {
        let size = file.content.len() as u64;
        let encoding = detect_encoding(&file.content, &file.mime_type);
        let metadata = FileMetadata {
            name: file.filename,
            size,
            processing_time_ms: 0,
            encoding,
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
    if is_text_mime_type(mime_type) {
        let mut detector = EncodingDetector::new();
        detector.feed(content, true);
        let encoding = detector.guess(None, true);
        encoding.name().to_string()
    } else {
        "application/octet-stream".to_string()
    }
}

fn is_text_mime_type(mime_type: &str) -> bool {
    mime_type.starts_with("text/")
        || matches!(
            mime_type,
            "application/json"
                | "application/xml"
                | "application/javascript"
                | "application/typescript"
                | "application/x-javascript"
                | "application/xhtml+xml"
                | "application/ld+json"
        )
}
