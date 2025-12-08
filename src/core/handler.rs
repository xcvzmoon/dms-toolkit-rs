/// Trait defining the contract for file handlers that extract text from different file formats.
///
/// This trait is the core abstraction that allows the system to support multiple file types
/// in a uniform way. Each file type handler (TextHandler, PdfHandler, etc.) implements this
/// trait to provide format-specific text extraction logic.
///
/// # Thread Safety
///
/// The trait requires `Send + Sync` bounds, ensuring that handlers can be safely shared
/// across multiple threads. This is essential for parallel file processing using Rayon.
///
/// # Implementation Pattern
///
/// Handlers typically:
/// 1. Check if they can handle a file type using `can_handle()`
/// 2. Extract text content using `extract_text()` if they can handle the file
/// 3. Return extracted text or an error message
///
/// # Example
///
/// ```no_run
/// use crate::core::handler::FileHandler;
///
/// struct MyHandler;
///
/// impl FileHandler for MyHandler {
///     fn can_handle(&self, mime_type: &str) -> bool {
///         mime_type == "application/my-format"
///     }
///
///     fn extract_text(&self, content: &[u8], _filename: &str, _mime_type: &str) -> Result<String, String> {
///         // Extract text from content
///         Ok("extracted text".to_string())
///     }
/// }
/// ```
pub trait FileHandler: Send + Sync {
    /// Checks whether this handler can process files of the given MIME type.
    ///
    /// This method is called by the processing system to determine which handler
    /// should be used for a particular file. The first handler that returns `true`
    /// for a given MIME type will be used to process that file.
    ///
    /// # Arguments
    ///
    /// * `mime_type` - The MIME type string (e.g., "application/pdf", "text/plain")
    ///
    /// # Returns
    ///
    /// `true` if this handler can process files of the given MIME type, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use crate::core::handler::FileHandler;
    /// # struct PdfHandler;
    /// # impl FileHandler for PdfHandler {
    /// #     fn can_handle(&self, mime_type: &str) -> bool {
    /// assert!(handler.can_handle("application/pdf"));
    /// assert!(!handler.can_handle("text/plain"));
    /// #     }
    /// #     fn extract_text(&self, _: &[u8], _: &str, _: &str) -> Result<String, String> {
    /// #         Ok(String::new())
    /// #     }
    /// # }
    /// ```
    fn can_handle(&self, mime_type: &str) -> bool;

    /// Extracts text content from the given file bytes.
    ///
    /// This method performs the actual text extraction from the raw file content.
    /// The implementation is format-specific and may involve parsing, decoding,
    /// or other format-specific operations.
    ///
    /// # Arguments
    ///
    /// * `content` - The raw file content as a byte slice
    /// * `filename` - The name of the file (may be used for logging or format detection)
    /// * `mime_type` - The MIME type of the file (already verified by `can_handle()`)
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Successfully extracted text content
    /// * `Err(String)` - Error message describing what went wrong during extraction
    ///
    /// # Error Handling
    ///
    /// Handlers should return descriptive error messages that help users understand
    /// what went wrong. Common error scenarios include:
    /// - Invalid file format or corrupted file
    /// - Unsupported file version or features
    /// - Encoding/decoding failures
    /// - Missing dependencies or resources
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use crate::core::handler::FileHandler;
    /// # struct TextHandler;
    /// # impl FileHandler for TextHandler {
    /// #     fn can_handle(&self, _: &str) -> bool { true }
    /// let content = b"Hello, world!";
    /// match handler.extract_text(content, "file.txt", "text/plain") {
    ///     Ok(text) => println!("Extracted: {}", text),
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// #     fn extract_text(&self, _: &[u8], _: &str, _: &str) -> Result<String, String> {
    /// #         Ok(String::new())
    /// #     }
    /// # }
    /// ```
    fn extract_text(
        &self,
        content: &[u8],
        filename: &str,
        mime_type: &str,
    ) -> Result<String, String>;
}
