//! Image file handler for extracting text from images using OCR.
//!
//! This handler uses OCR (Optical Character Recognition) to detect and extract
//! text from images. It uses pre-trained models for text detection and recognition.

use crate::core::handler::FileHandler;
use image::ImageReader;
use rten::Model;
use std::io::Cursor;
use std::path::PathBuf;

/// Handler for processing image files and extracting text using OCR.
///
/// The `ImageHandler` uses OCR (Optical Character Recognition) to extract text
/// from images. It requires pre-trained models for text detection and recognition
/// that are loaded at handler initialization.
///
/// # Supported MIME Types
///
/// - `image/jpeg` / `image/jpg` - JPEG images
/// - `image/png` - PNG images
/// - `image/gif` - GIF images
/// - `image/bmp` - BMP images
/// - `image/tiff` - TIFF images
/// - `image/webp` - WebP images
///
/// # Processing Flow
///
/// 1. **Image Loading**: Reads and decodes the image from bytes
/// 2. **Format Conversion**: Converts the image to RGB8 format for OCR processing
/// 3. **Text Detection**: Uses the detection model to identify regions containing text
///    (word bounding boxes)
/// 4. **Text Line Finding**: Groups detected words into text lines
/// 5. **Text Recognition**: Uses the recognition model to convert detected text
///    regions into actual text strings
/// 6. **Text Assembly**: Combines all recognized text lines with newlines
///
/// # Model Requirements
///
/// The handler requires two model files in the project root:
/// - `text-detection-model.rten` - Model for detecting text regions in images
/// - `text-recognition-model.rten` - Model for recognizing text in detected regions
///
/// These models are loaded once when the handler is created and reused for all
/// image processing operations.
///
/// # Limitations
///
/// - OCR accuracy depends on image quality, resolution, and text clarity
/// - Handwritten text may not be recognized accurately
/// - Complex layouts or rotated text may reduce accuracy
/// - Processing time increases with image size
pub struct ImageHandler {
    /// The OCR engine containing detection and recognition models.
    model: ocrs::OcrEngine,
}

impl ImageHandler {
    /// Creates a new `ImageHandler` instance.
    ///
    /// This method loads the required OCR models from files in the project root.
    /// The models are loaded once and reused for all subsequent image processing.
    ///
    /// # Returns
    ///
    /// A new `ImageHandler` ready to process image files.
    ///
    /// # Panics
    ///
    /// This method will panic if:
    /// - The model files cannot be found in the project root
    /// - The model files are corrupted or invalid
    /// - The OCR engine cannot be initialized
    ///
    /// # Model Files
    ///
    /// Expects the following files in the project root (same directory as Cargo.toml):
    /// - `text-detection-model.rten`
    /// - `text-recognition-model.rten`
    pub fn new() -> Self {
        let detection_model_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("text-detection-model.rten");
        let recognition_model_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("text-recognition-model.rten");

        let detection_model =
            Model::load_file(detection_model_path).expect("Failed to load detection model");
        let recognition_model =
            Model::load_file(recognition_model_path).expect("Failed to load recognition model");

        let model = ocrs::OcrEngine::new(ocrs::OcrEngineParams {
            detection_model: Some(detection_model),
            recognition_model: Some(recognition_model),
            ..Default::default()
        })
        .expect("Failed to initialize OCR engine");

        Self { model }
    }

    /// Extracts text from an image using OCR.
    ///
    /// This method performs the complete OCR pipeline: image loading, text detection,
    /// line finding, text recognition, and result assembly.
    ///
    /// # Arguments
    ///
    /// * `content` - The raw image file content as a byte slice
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Successfully extracted text content, or "No text found in image"
    ///   if no text was detected
    /// * `Err(String)` - Error message if any step fails:
    ///   - "Failed to read image: ..." - Image loading/decoding error
    ///   - "Failed to create image source: ..." - Image format conversion error
    ///   - "Failed to prepare OCR input: ..." - OCR input preparation error
    ///   - "Failed to detect words: ..." - Text detection error
    ///   - "OCR recognition failed: ..." - Text recognition error
    ///
    /// # Processing Steps
    ///
    /// 1. **Image Decoding**: Uses `image` crate to decode the image bytes
    /// 2. **Format Conversion**: Converts to RGB8 format required by OCR engine
    /// 3. **Input Preparation**: Prepares the image for OCR processing
    /// 4. **Word Detection**: Detects bounding boxes for text regions (words)
    /// 5. **Line Finding**: Groups words into text lines
    /// 6. **Text Recognition**: Recognizes text in each line
    /// 7. **Result Assembly**: Combines recognized lines with newlines
    ///
    /// # Output Format
    ///
    /// Each recognized text line is separated by a newline character. Empty lines
    /// (after trimming) are filtered out. If no text is found, returns "No text found in image".
    fn extract_text_from_image(&self, content: &[u8]) -> Result<String, String> {
        let cursor = Cursor::new(content);
        let img = ImageReader::new(cursor)
            .with_guessed_format()
            .map_err(|e| format!("Failed to read image: {}", e))?
            .decode()
            .map_err(|e| format!("Failed to decode image: {}", e))?;

        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        let image_source = ocrs::ImageSource::from_bytes(rgb_img.as_raw(), (width, height))
            .map_err(|e| format!("Failed to create image source: {}", e))?;

        let ocr_input = self
            .model
            .prepare_input(image_source)
            .map_err(|e| format!("Failed to prepare OCR input: {}", e))?;

        let word_rects = self
            .model
            .detect_words(&ocr_input)
            .map_err(|e| format!("Failed to detect words: {}", e))?;

        let line_rects = self.model.find_text_lines(&ocr_input, &word_rects);

        let line_texts = self
            .model
            .recognize_text(&ocr_input, &line_rects)
            .map_err(|e| format!("OCR recognition failed: {}", e))?;

        let mut extracted_text = String::new();
        for line_text in line_texts {
            if let Some(text_line) = line_text {
                let text = text_line.to_string();
                if !text.trim().is_empty() {
                    extracted_text.push_str(&text);
                    extracted_text.push('\n');
                }
            }
        }

        let cleaned = extracted_text.trim().to_string();

        if cleaned.is_empty() {
            Ok("No text found in image".to_string())
        } else {
            Ok(cleaned)
        }
    }
}

impl FileHandler for ImageHandler {
    /// Determines if this handler can process image files.
    ///
    /// Returns `true` for supported image MIME types:
    /// - `image/jpeg` / `image/jpg`
    /// - `image/png`
    /// - `image/gif`
    /// - `image/bmp`
    /// - `image/tiff`
    /// - `image/webp`
    ///
    /// # Arguments
    ///
    /// * `mime_type` - The MIME type string to check
    ///
    /// # Returns
    ///
    /// `true` if the MIME type represents a supported image format, `false` otherwise.
    fn can_handle(&self, mime_type: &str) -> bool {
        mime_type.starts_with("image/")
            && (mime_type == "image/jpeg"
                || mime_type == "image/jpg"
                || mime_type == "image/png"
                || mime_type == "image/gif"
                || mime_type == "image/bmp"
                || mime_type == "image/tiff"
                || mime_type == "image/webp")
    }

    /// Extracts text content from an image using OCR.
    ///
    /// This is the main entry point for image text extraction. It delegates
    /// to `extract_text_from_image()` to perform the OCR processing.
    ///
    /// # Arguments
    ///
    /// * `content` - The raw image file content as a byte slice
    /// * `_filename` - The filename (unused, kept for trait compatibility)
    /// * `_mime_type` - The MIME type (unused, already verified by `can_handle()`)
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Successfully extracted text content from the image
    /// * `Err(String)` - Error message if OCR processing fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use crate::handlers::image::ImageHandler;
    /// # use crate::core::handler::FileHandler;
    /// let handler = ImageHandler::new();
    /// let image_bytes = vec![...]; // Image file bytes
    /// let text = handler.extract_text(&image_bytes, "image.png", "image/png");
    /// ```
    fn extract_text(
        &self,
        content: &[u8],
        _filename: &str,
        _mime_type: &str,
    ) -> Result<String, String> {
        self.extract_text_from_image(content)
    }
}
