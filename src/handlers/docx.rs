//! DOCX file handler for extracting text from Microsoft Word documents.
//!
//! This handler uses the `docx-rs` library to parse DOCX files (which are
//! ZIP archives containing XML files) and extract text content from them.

use crate::core::handler::FileHandler;
use docx_rs::*;

/// Handler for processing Microsoft Word documents (DOCX format).
///
/// The `DocxHandler` extracts text content from DOCX files. DOCX files are
/// actually ZIP archives containing XML files that define the document structure.
/// This handler navigates the document structure to extract text from paragraphs.
///
/// # Supported MIME Types
///
/// - `application/vnd.openxmlformats-officedocument.wordprocessingml.document` - Standard DOCX format
/// - `application/docx` - Alternative DOCX MIME type
///
/// # Processing Flow
///
/// 1. Parses the DOCX file structure using `docx-rs` library
/// 2. Iterates through document children (paragraphs)
/// 3. Extracts text from paragraph runs (text segments with formatting)
/// 4. Combines all text with newlines between paragraphs
/// 5. Trims leading/trailing whitespace
///
/// # Limitations
///
/// - Extracts plain text only (no formatting, images, tables, or complex elements)
/// - Does not preserve document structure or layout
/// - Only processes text from paragraphs (headers, footers, footnotes may be included)
pub struct DocxHandler;

impl DocxHandler {
    /// Creates a new `DocxHandler` instance.
    ///
    /// # Returns
    ///
    /// A new `DocxHandler` ready to process DOCX files.
    pub fn new() -> Self {
        Self
    }

    /// Extracts text content from a DOCX document.
    ///
    /// This method parses the DOCX file structure and extracts text from all
    /// paragraphs in the document. DOCX files are ZIP archives containing XML,
    /// and this method navigates the XML structure to find text content.
    ///
    /// # Arguments
    ///
    /// * `content` - The raw DOCX file content as a byte slice
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Successfully extracted text content with newlines between paragraphs
    /// * `Err(String)` - Error message if parsing fails (e.g., "Failed to read DOCX: ...")
    ///
    /// # Error Conditions
    ///
    /// Returns an error if:
    /// - The DOCX file is corrupted or invalid
    /// - The file is not a valid DOCX format
    /// - Parsing the document structure fails
    ///
    /// # Text Extraction Details
    ///
    /// The method:
    /// - Iterates through all document children (typically paragraphs)
    /// - For each paragraph, extracts text from runs (formatted text segments)
    /// - Combines text from all runs in a paragraph
    /// - Adds a newline after each paragraph
    /// - Trims the final result to remove leading/trailing whitespace
    fn extract_text_from_docx(&self, content: &[u8]) -> Result<String, String> {
        let docx = read_docx(&content).map_err(|e| format!("Failed to read DOCX: {}", e))?;

        let mut text = String::new();

        for child in docx.document.children {
            if let DocumentChild::Paragraph(para) = child {
                for run in para.children {
                    if let ParagraphChild::Run(run_child) = run {
                        for run_content in run_child.children {
                            if let RunChild::Text(text_node) = run_content {
                                text.push_str(&text_node.text);
                            }
                        }
                    }
                }
                text.push('\n');
            }
        }

        Ok(text.trim().to_string())
    }
}

impl FileHandler for DocxHandler {
    /// Determines if this handler can process DOCX files.
    ///
    /// Returns `true` for standard DOCX MIME types:
    /// - `application/vnd.openxmlformats-officedocument.wordprocessingml.document`
    /// - `application/docx`
    ///
    /// # Arguments
    ///
    /// * `mime_type` - The MIME type string to check
    ///
    /// # Returns
    ///
    /// `true` if the MIME type represents a DOCX file, `false` otherwise.
    fn can_handle(&self, mime_type: &str) -> bool {
        mime_type == "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            || mime_type == "application/docx"
    }

    /// Extracts text content from a DOCX document.
    ///
    /// This is the main entry point for DOCX text extraction. It delegates
    /// to `extract_text_from_docx()` to perform the actual extraction.
    ///
    /// # Arguments
    ///
    /// * `content` - The raw DOCX file content as a byte slice
    /// * `_filename` - The filename (unused, kept for trait compatibility)
    /// * `_mime_type` - The MIME type (unused, already verified by `can_handle()`)
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Successfully extracted text content
    /// * `Err(String)` - Error message if extraction fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use crate::handlers::docx::DocxHandler;
    /// # use crate::core::handler::FileHandler;
    /// let handler = DocxHandler::new();
    /// let docx_bytes = vec![...]; // DOCX file bytes
    /// let text = handler.extract_text(&docx_bytes, "document.docx", "application/vnd.openxmlformats-officedocument.wordprocessingml.document");
    /// ```
    fn extract_text(
        &self,
        content: &[u8],
        _filename: &str,
        _mime_type: &str,
    ) -> Result<String, String> {
        self.extract_text_from_docx(content)
    }
}
