use crate::core::handler::FileHandler;
use docx_rs::*;

pub struct DocxHandler;

impl DocxHandler {
    pub fn new() -> Self {
        Self
    }

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
    fn can_handle(&self, mime_type: &str) -> bool {
        mime_type == "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            || mime_type == "application/docx"
    }

    fn extract_text(
        &self,
        content: &[u8],
        _filename: &str,
        _mime_type: &str,
    ) -> Result<String, String> {
        self.extract_text_from_docx(content)
    }
}
