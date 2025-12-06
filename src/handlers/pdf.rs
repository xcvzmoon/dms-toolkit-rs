use crate::core::handler::FileHandler;
use pdf_extract::extract_text_from_mem;

pub struct PdfHandler;

impl PdfHandler {
    pub fn new() -> Self {
        Self
    }
}

impl FileHandler for PdfHandler {
    fn can_handle(&self, mime_type: &str) -> bool {
        mime_type == "application/pdf"
    }

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
