use crate::core::handler::FileHandler;
use image::ImageReader;
use rten::Model;
use std::io::Cursor;
use std::path::PathBuf;

pub struct ImageHandler {
    model: ocrs::OcrEngine,
}

impl ImageHandler {
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

    fn extract_text(
        &self,
        content: &[u8],
        _filename: &str,
        _mime_type: &str,
    ) -> Result<String, String> {
        self.extract_text_from_image(content)
    }
}
