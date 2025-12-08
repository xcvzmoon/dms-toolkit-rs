//! Data structures for file processing input and output.
//!
//! This module defines the data structures used for communication between
//! Node.js and the Rust library via NAPI bindings.

use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

/// Input structure representing a file to be processed.
///
/// This structure is used as input to the `process_files` and
/// `process_and_compare_files` functions. It contains all the information
/// needed to process a file and extract its text content.
///
/// # Fields
///
/// * `content` - The raw file content as a Buffer (byte array)
/// * `mime_type` - The MIME type of the file (e.g., "application/pdf", "text/plain")
/// * `filename` - The name of the file (used for logging and error messages)
///
/// # Example
///
/// ```typescript
/// const file: FileInput = {
///   content: fs.readFileSync('document.pdf'),
///   mimeType: 'application/pdf',
///   filename: 'document.pdf'
/// };
/// ```
#[napi(object)]
pub struct FileInput {
    /// Raw file content as a Buffer (byte array).
    pub content: Buffer,
    /// MIME type identifying the file format.
    pub mime_type: String,
    /// Original filename of the file.
    pub filename: String,
}

/// Output structure representing processed file metadata.
///
/// This structure contains the results of processing a file, including
/// extracted text content and metadata about the processing operation.
///
/// # Fields
///
/// * `name` - The original filename
/// * `size` - File size in bytes (as a floating-point number)
/// * `processing_time_ms` - Time taken to process the file in milliseconds
///   (currently always 0.0, reserved for future use)
/// * `encoding` - Encoding information:
///   - "utf-8" for successfully processed files
///   - "error" for files where extraction failed
///   - "application/octet-stream" for unhandled file types
/// * `text_content` - The extracted text content, or an error message if extraction failed
///
/// # Example
///
/// ```typescript
/// const metadata: FileMetadata = {
///   name: 'document.pdf',
///   size: 1024.0,
///   processingTimeMs: 0.0,
///   encoding: 'utf-8',
///   textContent: 'Extracted text from PDF...'
/// };
/// ```
#[napi(object)]
pub struct FileMetadata {
    /// Original filename of the processed file.
    pub name: String,
    /// File size in bytes (floating-point number).
    pub size: f64,
    /// Processing time in milliseconds (currently always 0.0).
    pub processing_time_ms: f64,
    /// Encoding information: "utf-8" (success), "error" (failure), or "application/octet-stream" (unhandled).
    pub encoding: String,
    /// Extracted text content or error message.
    pub text_content: String,
}

/// Output structure representing files grouped by MIME type.
///
/// This structure is returned by `process_files` and organizes processed
/// files by their MIME type for easy access and processing.
///
/// # Fields
///
/// * `mime_type` - The MIME type that groups these files
/// * `files` - Array of `FileMetadata` objects for all files of this type
///
/// # Example
///
/// ```typescript
/// const grouped: GroupedFiles = {
///   mimeType: 'application/pdf',
///   files: [
///     { name: 'doc1.pdf', size: 1024, ... },
///     { name: 'doc2.pdf', size: 2048, ... }
///   ]
/// };
/// ```
#[napi(object)]
pub struct GroupedFiles {
    /// MIME type that groups these files together.
    pub mime_type: String,
    /// Array of processed file metadata for files of this MIME type.
    pub files: Vec<FileMetadata>,
}

/// Structure representing a similarity match between extracted text and a reference text.
///
/// This structure is used in similarity comparison results to indicate which
/// reference texts matched the extracted text and their similarity scores.
///
/// # Fields
///
/// * `reference_index` - The index of the reference text in the input array
///   (0-based, corresponds to the position in the `reference_texts` array)
/// * `similarity_percentage` - The similarity score as a percentage (0.0 to 100.0)
///
/// # Example
///
/// ```typescript
/// const match: SimilarityMatch = {
///   referenceIndex: 0,
///   similarityPercentage: 85.5
/// };
/// // Indicates the extracted text is 85.5% similar to reference_texts[0]
/// ```
#[napi(object)]
pub struct SimilarityMatch {
    /// Index of the reference text in the input array (0-based).
    pub reference_index: u32,
    /// Similarity percentage (0.0 to 100.0).
    pub similarity_percentage: f64,
}

/// Extended file metadata structure that includes similarity comparison results.
///
/// This structure extends `FileMetadata` with similarity match information.
/// It is returned by `process_and_compare_files` when files are processed
/// with similarity comparison enabled.
///
/// # Fields
///
/// All fields from `FileMetadata` plus:
/// * `similarity_matches` - Array of `SimilarityMatch` objects representing
///   reference texts that matched above the threshold
///
/// # Example
///
/// ```typescript
/// const metadata: FileMetadataWithSimilarity = {
///   name: 'document.pdf',
///   size: 1024.0,
///   processingTimeMs: 0.0,
///   encoding: 'utf-8',
///   textContent: 'Extracted text...',
///   similarityMatches: [
///     { referenceIndex: 0, similarityPercentage: 85.5 },
///     { referenceIndex: 2, similarityPercentage: 72.3 }
///   ]
/// };
/// ```
#[napi(object)]
pub struct FileMetadataWithSimilarity {
    /// Original filename of the processed file.
    pub name: String,
    /// File size in bytes (floating-point number).
    pub size: f64,
    /// Processing time in milliseconds (currently always 0.0).
    pub processing_time_ms: f64,
    /// Encoding information: "utf-8" (success), "error" (failure), or "application/octet-stream" (unhandled).
    pub encoding: String,
    /// Extracted text content or error message.
    pub text_content: String,
    /// Array of similarity matches above the threshold.
    pub similarity_matches: Vec<SimilarityMatch>,
}

/// Output structure representing files grouped by MIME type with similarity results.
///
/// This structure is returned by `process_and_compare_files` and organizes
/// processed files by their MIME type, with each file including similarity
/// comparison results.
///
/// # Fields
///
/// * `mime_type` - The MIME type that groups these files
/// * `files` - Array of `FileMetadataWithSimilarity` objects for all files of this type
///
/// # Example
///
/// ```typescript
/// const grouped: GroupedFilesWithSimilarity = {
///   mimeType: 'application/pdf',
///   files: [
///     {
///       name: 'doc1.pdf',
///       size: 1024,
///       ...,
///       similarityMatches: [...]
///     }
///   ]
/// };
/// ```
#[napi(object)]
pub struct GroupedFilesWithSimilarity {
    /// MIME type that groups these files together.
    pub mime_type: String,
    /// Array of processed file metadata with similarity matches for files of this MIME type.
    pub files: Vec<FileMetadataWithSimilarity>,
}
