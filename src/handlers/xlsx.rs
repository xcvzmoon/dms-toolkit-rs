//! XLSX file handler for extracting text from Microsoft Excel spreadsheets.
//!
//! This handler uses the `calamine` library to read Excel workbooks and extract
//! text content from all sheets and cells.

use crate::core::handler::FileHandler;
use calamine::{Reader, Xlsx, open_workbook_from_rs};
use std::io::Cursor;

/// Handler for processing Microsoft Excel spreadsheets (XLSX format).
///
/// The `XlsxHandler` extracts text content from XLSX files by reading all
/// sheets and converting cell values to text. Cells are separated by tabs
/// to preserve column structure, and rows are separated by newlines.
///
/// # Supported MIME Types
///
/// - `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet` - Standard XLSX format
/// - `application/vnd.ms-excel` - Legacy Excel format (also handled)
/// - `application/xlsx` - Alternative XLSX MIME type
///
/// # Processing Flow
///
/// 1. Opens the Excel workbook from memory using `calamine` library
/// 2. Iterates through all sheets in the workbook
/// 3. For each sheet:
///    - Adds a header line with the sheet name
///    - Processes each row in the sheet
///    - Converts all cell values to strings
///    - Filters out empty cells
///    - Joins cells with tab characters (preserving column structure)
///    - Adds a newline after each row
/// 4. Separates sheets with double newlines
/// 5. Trims the final output
///
/// # Output Format
///
/// The extracted text follows this structure:
/// ```
/// Sheet: Sheet1
/// Cell1    Cell2    Cell3
/// Value1   Value2   Value3
///
/// Sheet: Sheet2
/// ...
/// ```
///
/// # Limitations
///
/// - Extracts text values only (formulas are converted to their calculated values)
/// - Does not preserve formatting, colors, or styles
/// - Empty cells are filtered out (may affect column alignment in output)
pub struct XlsxHandler;

impl XlsxHandler {
    /// Creates a new `XlsxHandler` instance.
    ///
    /// # Returns
    ///
    /// A new `XlsxHandler` ready to process XLSX files.
    pub fn new() -> Self {
        Self
    }

    /// Extracts text content from an XLSX spreadsheet.
    ///
    /// This method processes all sheets in the workbook and converts cell
    /// values to text, preserving the row/column structure with tabs and newlines.
    ///
    /// # Arguments
    ///
    /// * `content` - The raw XLSX file content as a byte slice
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Successfully extracted text content with sheet headers and cell values
    /// * `Err(String)` - Error message if parsing fails (e.g., "Failed to open Excel file: ...")
    ///
    /// # Error Conditions
    ///
    /// Returns an error if:
    /// - The XLSX file is corrupted or invalid
    /// - The file is not a valid XLSX format
    /// - Opening or reading the workbook fails
    ///
    /// # Cell Value Conversion
    ///
    /// All cell values are converted to strings using their `to_string()` method.
    /// This means:
    /// - Numbers are converted to their string representation
    /// - Dates are converted to their string format
    /// - Formulas are converted to their calculated values
    /// - Empty cells are filtered out
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
    /// Determines if this handler can process XLSX files.
    ///
    /// Returns `true` for Excel spreadsheet MIME types:
    /// - `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet` (standard XLSX)
    /// - `application/vnd.ms-excel` (legacy Excel format)
    /// - `application/xlsx` (alternative MIME type)
    ///
    /// # Arguments
    ///
    /// * `mime_type` - The MIME type string to check
    ///
    /// # Returns
    ///
    /// `true` if the MIME type represents an Excel file, `false` otherwise.
    fn can_handle(&self, mime_type: &str) -> bool {
        mime_type == "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            || mime_type == "application/vnd.ms-excel"
            || mime_type == "application/xlsx"
    }

    /// Extracts text content from an XLSX spreadsheet.
    ///
    /// This is the main entry point for XLSX text extraction. It delegates
    /// to `extract_text_from_xlsx()` to perform the actual extraction.
    ///
    /// # Arguments
    ///
    /// * `content` - The raw XLSX file content as a byte slice
    /// * `_filename` - The filename (unused, kept for trait compatibility)
    /// * `_mime_type` - The MIME type (unused, already verified by `can_handle()`)
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Successfully extracted text content with all sheets and cells
    /// * `Err(String)` - Error message if extraction fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use crate::handlers::xlsx::XlsxHandler;
    /// # use crate::core::handler::FileHandler;
    /// let handler = XlsxHandler::new();
    /// let xlsx_bytes = vec![...]; // XLSX file bytes
    /// let text = handler.extract_text(&xlsx_bytes, "spreadsheet.xlsx", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet");
    /// ```
    fn extract_text(
        &self,
        content: &[u8],
        _filename: &str,
        _mime_type: &str,
    ) -> Result<String, String> {
        self.extract_text_from_xlsx(content)
    }
}
