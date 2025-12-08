//! Text file handler for processing plain text and text-based file formats.
//!
//! This handler supports various text-based MIME types and automatically detects
//! character encoding to properly decode text content.

use crate::core::handler::FileHandler;
use chardetng::EncodingDetector;
use encoding_rs::Encoding;

/// Handler for processing text files and text-based formats.
///
/// The `TextHandler` is responsible for extracting text from plain text files
/// and other text-based formats (JSON, XML, CSV, etc.). It automatically detects
/// the character encoding of text files to ensure proper decoding.
///
/// # Supported MIME Types
///
/// - `text/*` - All text MIME types (text/plain, text/html, text/css, etc.)
/// - `text/csv` - Comma-separated values files
/// - `text/tsv` - Tab-separated values files
/// - `text/tab-separated-values` - Alternative TSV MIME type
/// - `application/json` - JSON files
/// - `application/xml` - XML files
/// - `application/javascript` - JavaScript files
/// - `application/typescript` - TypeScript files
/// - `application/x-javascript` - Alternative JavaScript MIME type
/// - `application/xhtml+xml` - XHTML files
/// - `application/ld+json` - JSON-LD files
///
/// # Processing Flow
///
/// 1. Detects the character encoding of the file content
/// 2. Decodes the bytes using the detected encoding
/// 3. Returns the decoded text content
///
/// # Error Handling
///
/// If decoding fails (e.g., invalid encoding or corrupted content), the handler
/// returns an error message indicating the failure.
pub struct TextHandler;

impl TextHandler {
    /// Creates a new `TextHandler` instance.
    ///
    /// # Returns
    ///
    /// A new `TextHandler` ready to process text files.
    pub fn new() -> Self {
        Self
    }

    /// Detects the character encoding of the given file content.
    ///
    /// Uses the `chardetng` library to analyze the byte content and determine
    /// the most likely character encoding. This is essential for properly decoding
    /// text files that may use various encodings (UTF-8, ISO-8859-1, Windows-1252, etc.).
    ///
    /// # Arguments
    ///
    /// * `content` - The raw file content as a byte slice
    ///
    /// # Returns
    ///
    /// The name of the detected encoding as a string (e.g., "utf-8", "iso-8859-1")
    ///
    /// # Algorithm
    ///
    /// The detection algorithm:
    /// 1. Feeds the content to an encoding detector
    /// 2. Analyzes byte patterns and statistical properties
    /// 3. Returns the most likely encoding name
    ///
    /// # Note
    ///
    /// Encoding detection is not 100% accurate, especially for short texts or
    /// texts with mixed content. The decoder will attempt to handle encoding errors
    /// gracefully.
    fn detect_encoding(&self, content: &[u8]) -> String {
        let mut detector = EncodingDetector::new();
        detector.feed(content, true);
        let encoding = detector.guess(None, true);
        encoding.name().to_string()
    }

    /// Checks if the given MIME type represents a text-based format.
    ///
    /// This method determines whether a MIME type should be handled as text,
    /// including both standard text MIME types and application MIME types that
    /// contain text content (JSON, XML, JavaScript, etc.).
    ///
    /// # Arguments
    ///
    /// * `mime_type` - The MIME type string to check
    ///
    /// # Returns
    ///
    /// `true` if the MIME type represents a text-based format, `false` otherwise.
    ///
    /// # Supported Patterns
    ///
    /// - Any MIME type starting with `text/` (e.g., text/plain, text/html)
    /// - Specific application MIME types: json, xml, javascript, typescript, xhtml, ld+json
    fn is_mime_type_text(&self, mime_type: &str) -> bool {
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

    /// Decodes byte content into a string using the specified encoding.
    ///
    /// Uses the `encoding_rs` library to decode bytes according to the given
    /// encoding name. If the encoding is not recognized, falls back to UTF-8.
    /// Handles decoding errors gracefully by returning an empty string if errors occur.
    ///
    /// # Arguments
    ///
    /// * `content` - The raw file content as a byte slice
    /// * `encoding_name` - The name of the encoding to use (e.g., "utf-8", "iso-8859-1")
    ///
    /// # Returns
    ///
    /// The decoded text as a `String`. Returns an empty string if decoding errors occur.
    ///
    /// # Error Handling
    ///
    /// If the encoding name is not recognized, the function falls back to UTF-8.
    /// If decoding errors occur (malformed sequences), the function returns an
    /// empty string. The caller should check for empty results when the content
    /// is known to be non-empty.
    fn decode_text(&self, content: &[u8], encoding_name: &str) -> String {
        let encoding = Encoding::for_label(encoding_name.as_bytes()).unwrap_or(encoding_rs::UTF_8);
        let (decoded, _encoding_used, had_errors) = encoding.decode(content);

        if had_errors {
            String::new()
        } else {
            decoded.to_string()
        }
    }
}

impl FileHandler for TextHandler {
    /// Determines if this handler can process files of the given MIME type.
    ///
    /// Returns `true` for text-based MIME types including:
    /// - All `text/*` MIME types
    /// - CSV and TSV files
    /// - Text-based application types (JSON, XML, JavaScript, etc.)
    ///
    /// # Arguments
    ///
    /// * `mime_type` - The MIME type string to check
    ///
    /// # Returns
    ///
    /// `true` if this handler can process the MIME type, `false` otherwise.
    fn can_handle(&self, mime_type: &str) -> bool {
        self.is_mime_type_text(mime_type)
            || mime_type == "text/csv"
            || mime_type == "text/tsv"
            || mime_type == "text/tab-separated-values"
    }

    /// Extracts text content from text-based file formats.
    ///
    /// This method performs the complete text extraction pipeline:
    /// 1. Detects the character encoding of the file
    /// 2. Decodes the bytes using the detected encoding
    /// 3. Returns the decoded text content
    ///
    /// # Arguments
    ///
    /// * `content` - The raw file content as a byte slice
    /// * `_filename` - The filename (unused, kept for trait compatibility)
    /// * `_mime_type` - The MIME type (unused, already verified by `can_handle()`)
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Successfully extracted and decoded text content
    /// * `Err(String)` - Error message if decoding fails (e.g., "Failed to decode text content")
    ///
    /// # Error Conditions
    ///
    /// Returns an error if:
    /// - The content is non-empty but decoding results in an empty string
    /// - Encoding detection or decoding fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use crate::handlers::text::TextHandler;
    /// # use crate::core::handler::FileHandler;
    /// let handler = TextHandler::new();
    /// let content = b"Hello, world!";
    /// let text = handler.extract_text(content, "file.txt", "text/plain");
    /// assert!(text.is_ok());
    /// ```
    fn extract_text(
        &self,
        content: &[u8],
        _filename: &str,
        _mime_type: &str,
    ) -> Result<String, String> {
        let encoding = self.detect_encoding(content);
        let text = self.decode_text(content, &encoding);

        if text.is_empty() && !content.is_empty() {
            Err("Failed to decode text content".to_string())
        } else {
            Ok(text)
        }
    }
}
