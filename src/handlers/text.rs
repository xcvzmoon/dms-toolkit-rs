use crate::core::handler::FileHandler;
use crate::core::text::{decode_text, is_mime_type_text};
use chardetng::EncodingDetector;

pub struct TextHandler;

impl TextHandler {
    pub fn new() -> Self {
        Self
    }

    fn detect_encoding(&self, content: &[u8]) -> String {
        let mut detector = EncodingDetector::new();
        detector.feed(content, true);
        let encoding = detector.guess(None, true);
        encoding.name().to_string()
    }
}

impl FileHandler for TextHandler {
    fn can_handle(&self, mime_type: &str) -> bool {
        is_mime_type_text(mime_type)
            || mime_type == "text/csv"
            || mime_type == "text/tsv"
            || mime_type == "text/tab-separated-values"
    }

    fn extract_text(
        &self,
        content: &[u8],
        _filename: &str,
        _mime_type: &str,
    ) -> Result<String, String> {
        let encoding = self.detect_encoding(content);
        let text = decode_text(content, &encoding);

        if text.is_empty() && !content.is_empty() {
            Err("Failed to decode text content".to_string())
        } else {
            Ok(text)
        }
    }
}
