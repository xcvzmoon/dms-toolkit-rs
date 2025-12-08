mod core;
mod handlers;
mod models;

use crate::core::handler::FileHandler;
use crate::core::similarity::{SimilarityMethod, compare_with_documents};

use crate::handlers::docx::DocxHandler;
use crate::handlers::image::ImageHandler;
use crate::handlers::pdf::PdfHandler;
use crate::handlers::text::TextHandler;
use crate::handlers::xlsx::XlsxHandler;
use crate::models::file::FileMetadataWithSimilarity;

use dashmap::DashMap;
use models::file::{
    FileInput, FileMetadata, GroupedFiles, GroupedFilesWithSimilarity, SimilarityMatch,
};
use napi_derive::napi;
use rayon::prelude::*;
use std::sync::Arc;

#[napi]
pub fn process_files(files: Vec<FileInput>) -> Vec<GroupedFiles> {
    let handlers: Vec<Arc<dyn FileHandler>> = vec![
        Arc::new(DocxHandler::new()),
        Arc::new(ImageHandler::new()),
        Arc::new(PdfHandler::new()),
        Arc::new(TextHandler::new()),
        Arc::new(XlsxHandler::new()),
    ];

    let grouped: DashMap<String, Vec<FileMetadata>> = DashMap::new();

    files.par_iter().for_each(|file| {
        let content = file.content.as_ref();
        let size = content.len() as f64;

        let handler = handlers.iter().find(|h| h.can_handle(&file.mime_type));

        let (text_content, encoding) = match handler {
            Some(h) => match h.extract_text(content, &file.filename, &file.mime_type) {
                Ok(text) => (text, "utf-8".to_string()),
                Err(err) => (format!("Error: {}", err), "error".to_string()),
            },
            None => (String::new(), "application/octet-stream".to_string()),
        };

        let metadata = FileMetadata {
            name: file.filename.clone(),
            size,
            processing_time_ms: 0.0,
            encoding,
            text_content,
        };

        grouped
            .entry(file.mime_type.clone())
            .or_insert_with(Vec::new)
            .push(metadata);
    });

    grouped
        .into_iter()
        .map(|(mime_type, files)| GroupedFiles { mime_type, files })
        .collect()
}

#[napi]
pub fn process_and_compare_files(
    files: Vec<FileInput>,
    reference_texts: Vec<String>,
    similarity_threshold: Option<f64>,
    similarity_method: Option<String>,
) -> Vec<GroupedFilesWithSimilarity> {
    let threshold = similarity_threshold.unwrap_or(30.0);

    // Parse similarity method
    let method = match similarity_method.as_deref() {
        Some("jaccard") => SimilarityMethod::Jaccard,
        Some("ngram") => SimilarityMethod::Ngram,
        Some("levenshtein") => SimilarityMethod::Levenshtein,
        Some("hybrid") | _ => SimilarityMethod::Hybrid,
    };

    // Initialize handlers
    let handlers: Vec<Arc<dyn FileHandler>> = vec![
        Arc::new(TextHandler::new()),
        Arc::new(PdfHandler::new()),
        Arc::new(DocxHandler::new()),
        Arc::new(XlsxHandler::new()),
        Arc::new(ImageHandler::new()),
    ];

    // Thread-safe concurrent HashMap for grouping
    let grouped: DashMap<String, Vec<FileMetadataWithSimilarity>> = DashMap::new();

    // Process files in parallel
    files.par_iter().for_each(|file| {
        let content = file.content.as_ref();
        let size = content.len() as f64;

        // Find appropriate handler
        let handler = handlers.iter().find(|h| h.can_handle(&file.mime_type));

        let (text_content, encoding) = match handler {
            Some(h) => match h.extract_text(content, &file.filename, &file.mime_type) {
                Ok(text) => (text, "utf-8".to_string()),
                Err(err) => (format!("Error: {}", err), "error".to_string()),
            },
            None => (String::new(), "application/octet-stream".to_string()),
        };

        // Compare with reference texts (only if text was extracted successfully)
        let similarity_matches = if !text_content.is_empty() && !text_content.starts_with("Error:")
        {
            let matches =
                compare_with_documents(&text_content, &reference_texts, method, threshold);

            matches
                .into_iter()
                .map(|(idx, similarity)| SimilarityMatch {
                    reference_index: idx as u32,
                    similarity_percentage: similarity,
                })
                .collect()
        } else {
            Vec::new()
        };

        let metadata = FileMetadataWithSimilarity {
            name: file.filename.clone(),
            size,
            processing_time_ms: 0.0,
            encoding,
            text_content,
            similarity_matches,
        };

        grouped
            .entry(file.mime_type.clone())
            .or_insert_with(Vec::new)
            .push(metadata);
    });

    // Convert DashMap to Vec<GroupedFilesWithSimilarity>
    grouped
        .into_iter()
        .map(|(mime_type, files)| GroupedFilesWithSimilarity { mime_type, files })
        .collect()
}
