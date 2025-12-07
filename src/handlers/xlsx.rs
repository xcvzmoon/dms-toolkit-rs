use crate::core::handler::FileHandler;
use calamine::{Reader, Xlsx, open_workbook_from_rs};
use std::io::Cursor;

pub struct XlsxHandler;

impl XlsxHandler {
    pub fn new() -> Self {
        Self
    }

    fn extract_text_from_xlsx(&self, content: &[u8]) -> Result<String, String> {
        let cursor = Cursor::new(content);
        let mut workbook: Xlsx<_> = open_workbook_from_rs(cursor)
            .map_err(|e| format!("Failed to open Excel file: {}", e))?;

        let mut text = String::new();

        let sheet_names = workbook.sheet_names().to_vec();

        for sheet_name in sheet_names {
            if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                if !text.is_empty() {
                    text.push_str("\n\n");
                }

                text.push_str(&format!("Sheet: {}\n", sheet_name));

                for row in range.rows() {
                    let row_text: Vec<String> = row
                        .iter()
                        .map(|cell| cell.to_string())
                        .filter(|s| !s.is_empty())
                        .collect();

                    if !row_text.is_empty() {
                        text.push_str(&row_text.join("\t"));
                        text.push('\n');
                    }
                }
            }
        }

        Ok(text.trim().to_string())
    }
}

impl FileHandler for XlsxHandler {
    fn can_handle(&self, mime_type: &str) -> bool {
        mime_type == "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            || mime_type == "application/vnd.ms-excel"
            || mime_type == "application/xlsx"
    }

    fn extract_text(
        &self,
        content: &[u8],
        _filename: &str,
        _mime_type: &str,
    ) -> Result<String, String> {
        self.extract_text_from_xlsx(content)
    }
}
