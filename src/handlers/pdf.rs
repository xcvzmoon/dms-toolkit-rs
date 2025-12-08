//! PDF file handler for extracting text from PDF documents.
//!
//! This handler uses the `pdf-extract` library to parse PDF files and extract
//! readable text content from them.

use crate::core::handler::FileHandler;
use pdf_extract::extract_text_from_mem;

/// Handler for processing PDF (Portable Document Format) files.
///
/// The `PdfHandler` extracts text content from PDF documents. It handles
/// standard PDF files and cleans up the extracted text by removing excessive
/// whitespace and empty lines.
///
/// # Supported MIME Types
///
/// - `application/pdf` - Standard PDF documents
///
/// # Processing Flow
///
/// 1. Uses `pdf-extract` library to extract raw text from PDF bytes
/// 2. Cleans the extracted text:
///    - Trims whitespace from each line
///    - Removes empty lines
///    - Joins lines with newline characters
/// 3. Returns the cleaned text content
///
/// # Limitations
///
/// - Extracts text only (no images, tables, or complex layouts)
/// - May not preserve exact formatting or structure
/// - Scanned PDFs (image-based) require OCR and should use ImageHandler instead
pub struct PdfHandler;

impl PdfHandler {
    /// Creates a new `PdfHandler` instance.
    ///
    /// # Returns
    ///
    /// A new `PdfHandler` ready to process PDF files.
    pub fn new() -> Self {
        Self
    }
}

impl FileHandler for PdfHandler {
    /// Determines if this handler can process PDF files.
    ///
    /// Returns `true` only for `application/pdf` MIME type.
    ///
    /// # Arguments
    ///
    /// * `mime_type` - The MIME type string to check
    ///
    /// # Returns
    ///
    /// `true` if the MIME type is `application/pdf`, `false` otherwise.
    fn can_handle(&self, mime_type: &str) -> bool {
        mime_type == "application/pdf"
    }

    /// Extracts text content from a PDF document.
    ///
    /// This method extracts text from PDF files loaded in memory and performs
    /// cleanup to produce readable text output.
    ///
    /// # Arguments
    ///
    /// * `content` - The raw PDF file content as a byte slice
    /// * `_filename` - The filename (unused, kept for trait compatibility)
    /// * `_mime_type` - The MIME type (unused, already verified by `can_handle()`)
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Successfully extracted and cleaned text content
    /// * `Err(String)` - Error message if extraction fails (e.g., "PDF extraction failed: ...")
    ///
    /// # Error Conditions
    ///
    /// Returns an error if:
    /// - The PDF file is corrupted or invalid
    /// - The PDF format is not supported
    /// - Text extraction fails for any reason
    ///
    /// # Text Cleaning
    ///
    /// The extracted text is cleaned by:
    /// - Trimming whitespace from the beginning and end of each line
    /// - Removing completely empty lines
    /// - Joining non-empty lines with newline characters
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use crate::handlers::pdf::PdfHandler;
    /// # use crate::core::handler::FileHandler;
    /// let handler = PdfHandler::new();
    /// let pdf_bytes = vec![...]; // PDF file bytes
    /// let text = handler.extract_text(&pdf_bytes, "document.pdf", "application/pdf");
    /// ```
    fn extract_text(
        &self,
        content: &[u8],
        _filename: &str,
        _mime_type: &str,
    ) -> Result<String, String> {
        match extract_text_from_mem(content) {
            Ok(text) => {
                // Clean up the extracted text (remove excessive whitespace)
                let cleaned = text
                    .lines()
                    .map(|line| line.trim())
                    .filter(|line| !line.is_empty())
                    .collect::<Vec<_>>()
                    .join("\n");

                Ok(cleaned)
            }
            Err(e) => Err(format!("PDF extraction failed: {}", e)),
        }
    }
}
