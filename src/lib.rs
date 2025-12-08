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

/// Processes an array of files and extracts text content from them.
///
/// This function takes a list of files with their MIME types and filenames,
/// processes them in parallel using appropriate handlers, and returns the
/// extracted text content grouped by MIME type.
///
/// # Supported File Types
///
/// - Text files (text/plain, text/csv, text/tsv, and other text-based MIME types)
/// - PDF documents (application/pdf)
/// - Microsoft Word documents (DOCX format)
/// - Excel spreadsheets (XLSX format)
/// - Images with OCR support (PNG, JPEG, GIF, BMP, TIFF, WebP)
///
/// # Processing Flow
///
/// 1. Initializes all available file handlers
/// 2. For each file, finds the appropriate handler based on MIME type
/// 3. Extracts text content using the handler's extraction logic
/// 4. Groups results by MIME type for easy access
/// 5. Returns grouped results with metadata for each file
///
/// # Parallel Processing
///
/// Files are processed in parallel using Rayon, which automatically utilizes
/// all available CPU cores. This significantly improves performance when
/// processing multiple files.
///
/// # Error Handling
///
/// If a file cannot be processed (no handler found, extraction fails, etc.),
/// the function still includes it in the results with:
/// - `encoding` set to "error" or "application/octet-stream"
/// - `text_content` containing an error message or empty string
///
/// # Arguments
///
/// * `files` - A vector of `FileInput` objects containing file content, MIME type, and filename
///
/// # Returns
///
/// A vector of `GroupedFiles` objects, where each group contains files of the same MIME type
/// along with their extracted text content and metadata.
///
/// # Example
///
/// ```no_run
/// use dms_toolkit_rs::process_files;
/// use dms_toolkit_rs::FileInput;
///
/// let files = vec![
///     FileInput {
///         content: vec![...], // PDF bytes
///         mime_type: "application/pdf".to_string(),
///         filename: "document.pdf".to_string(),
///     }
/// ];
///
/// let results = process_files(files);
/// ```
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

/// Processes files and compares extracted text against reference documents.
///
/// This function extends `process_files` by adding similarity comparison capabilities.
/// After extracting text from files, it compares each file's text content against
/// a list of reference texts using configurable similarity algorithms.
///
/// # Similarity Algorithms
///
/// The function supports multiple similarity methods:
///
/// - **"jaccard"**: Fast word-based similarity using Jaccard index. Best for quick
///   comparisons and initial filtering. Splits texts into words and calculates
///   intersection over union.
///
/// - **"ngram"**: Character n-gram based similarity (uses 3-grams). Good for
///   longer texts where word-based methods might miss character-level similarities.
///
/// - **"levenshtein"**: Edit distance based similarity. Calculates the minimum
///   number of edits needed to transform one string into another. More accurate
///   but slower for long texts.
///
/// - **"hybrid"** (default): Progressive filtering approach that combines multiple
///   methods for optimal balance of speed and accuracy:
///   1. Fast Jaccard check - if score < 20%, return immediately
///   2. For small texts (< 1000 chars): Use Levenshtein with early termination
///   3. For larger texts: Use N-gram similarity
///
/// # Processing Flow
///
/// 1. Processes files and extracts text content (same as `process_files`)
/// 2. For each successfully extracted text:
///    - Compares against all reference texts in parallel
///    - Applies pre-filtering using length heuristics
///    - Calculates similarity using the selected method
///    - Filters results by threshold (only matches >= threshold are returned)
/// 3. Returns grouped results with similarity match information
///
/// # Parallel Processing
///
/// Both file processing and similarity comparisons run in parallel:
/// - Multiple files are processed simultaneously
/// - Each file's text is compared against all reference texts in parallel
/// - Pre-filtering helps avoid expensive calculations for dissimilar texts
///
/// # Arguments
///
/// * `files` - A vector of `FileInput` objects to process
/// * `reference_texts` - A vector of reference text strings to compare against
/// * `similarity_threshold` - Optional similarity threshold percentage (0-100).
///   Defaults to 30.0. Only matches with similarity >= threshold are returned.
/// * `similarity_method` - Optional similarity algorithm to use. Valid values:
///   "jaccard", "ngram", "levenshtein", "hybrid" (default). Invalid values
///   default to "hybrid".
///
/// # Returns
///
/// A vector of `GroupedFilesWithSimilarity` objects, where each group contains:
/// - Files grouped by MIME type
/// - Extracted text content and metadata
/// - Similarity matches for each file (reference index and similarity percentage)
///
/// # Example
///
/// ```no_run
/// use dms_toolkit_rs::process_and_compare_files;
/// use dms_toolkit_rs::FileInput;
///
/// let files = vec![
///     FileInput {
///         content: vec![...], // PDF bytes
///         mime_type: "application/pdf".to_string(),
///         filename: "document.pdf".to_string(),
///     }
/// ];
///
/// let reference_texts = vec![
///     "This is a reference document.".to_string(),
///     "Another reference text.".to_string(),
/// ];
///
/// let results = process_and_compare_files(
///     files,
///     reference_texts,
///     Some(30.0),  // 30% threshold
///     Some("hybrid".to_string()),  // Use hybrid method
/// );
/// ```
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
