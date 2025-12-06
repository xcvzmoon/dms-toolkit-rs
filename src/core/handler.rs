pub trait FileHandler: Send + Sync {
    fn can_handle(&self, mime_type: &str) -> bool;
    fn extract_text(
        &self,
        content: &[u8],
        filename: &str,
        mime_type: &str,
    ) -> Result<String, String>;
}
