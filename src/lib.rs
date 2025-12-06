mod core;
mod handlers;
mod models;

use crate::core::handler::FileHandler;
use crate::handlers::text::TextHandler;
use models::file::{FileInput, FileMetadata, GroupedFiles};
use napi_derive::napi;
use std::collections::HashMap;

#[napi]
pub fn process_files(files: Vec<FileInput>) -> Vec<GroupedFiles> {
    let mut grouped: HashMap<String, Vec<FileMetadata>> = HashMap::new();
    let text_handler = TextHandler::new();
    let handlers: Vec<Box<dyn FileHandler>> = vec![Box::new(text_handler)];

    for file in files {
        let content = file.content.as_ref();
        let size = content.len() as f64;
        let handler = handlers.iter().find(|h| h.can_handle(&file.mime_type));

        let (text_content, encoding) = match handler {
            Some(h) => match h.extract_text(content, &file.filename, &file.mime_type) {
                Ok(text) => {
                    let enc = detect_encoding_for_metadata(content, &file.mime_type);
                    (text, enc)
                }
                Err(err) => (format!("Error: {}", err), "error".to_string()),
            },
            None => (String::new(), "application/octet-stream".to_string()),
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
            .or_insert_with(Vec::new)
            .push(metadata);
    }

    grouped
        .into_iter()
        .map(|(mime_type, files)| GroupedFiles { mime_type, files })
        .collect()
}

fn detect_encoding_for_metadata(content: &[u8], mime_type: &str) -> String {
    use crate::core::text::is_mime_type_text;
    use chardetng::EncodingDetector;

    if is_mime_type_text(mime_type) {
        let mut detector = EncodingDetector::new();
        detector.feed(content, true);
        let encoding = detector.guess(None, true);
        encoding.name().to_string()
    } else {
        "application/octet-stream".to_string()
    }
}
