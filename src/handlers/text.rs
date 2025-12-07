use crate::core::handler::FileHandler;
use chardetng::EncodingDetector;
use encoding_rs::Encoding;

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
    fn can_handle(&self, mime_type: &str) -> bool {
        self.is_mime_type_text(mime_type)
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
        let text = self.decode_text(content, &encoding);

        if text.is_empty() && !content.is_empty() {
            Err("Failed to decode text content".to_string())
        } else {
            Ok(text)
        }
    }
}
