# Architecture Documentation

This document explains the internal architecture and design of the DMS Toolkit codebase. It is written to be accessible to readers with no prior knowledge of Rust, explaining concepts as they appear.

## Overview

The DMS Toolkit is a file processing system that extracts text content from various file formats. The system is designed with a modular architecture where different file types are handled by specialized components called "handlers". Each handler knows how to process a specific file format (like PDF, Word documents, or text files) and extract readable text from it.

The main goal of the system is to take files of different types, process them in parallel for efficiency, and return the extracted text content along with metadata about each file. Files are automatically grouped by their MIME type (a standard way of identifying file formats) in the output.

## Core Concepts

Before diving into the architecture, here are some key concepts used throughout the codebase:

### Modules

In Rust, code is organized into modules (similar to packages or namespaces in other languages). Each module contains related functionality. The `src/` directory contains the main source code, and each subdirectory represents a module.

### Traits

A trait is like an interface or contract in other programming languages. It defines a set of methods that any type implementing the trait must provide. In this codebase, the `FileHandler` trait defines what methods a file handler must implement. This allows the system to work with different handler types in a uniform way.

### Structs

A struct is a data structure that groups related data together (similar to classes or objects in other languages). For example, `FileInput` is a struct that contains file content, MIME type, and filename.

### Implementation Blocks

In Rust, you define what a struct can do in an "implementation block" (written as `impl`). This is where methods and behavior are attached to a struct.

### Result Type

Rust uses a `Result` type to handle operations that might fail. A `Result` can be either `Ok(value)` for success or `Err(error)` for failure. This makes error handling explicit and safe.

## Project Structure

The codebase is organized into the following structure:

```
src/
├── core/           # Core functionality and shared contracts
│   ├── handler.rs  # The FileHandler trait definition
│   └── mod.rs      # Module declarations
├── handlers/       # Individual file type handlers
│   ├── text.rs     # Text file handler
│   ├── pdf.rs      # PDF file handler
│   ├── docx.rs     # Word document handler
│   ├── xlsx.rs     # Excel spreadsheet handler
│   └── mod.rs      # Module declarations
├── models/         # Data structures
│   ├── file.rs     # File input/output data structures
│   └── mod.rs      # Module declarations
└── lib.rs          # Main entry point and orchestration
```

## The Handler System

The handler system is the core design pattern used in this codebase. It allows the system to support multiple file formats in an extensible way.

### The FileHandler Trait

The `FileHandler` trait is defined in `src/core/handler.rs`. It serves as a contract that all file handlers must follow. Think of it as a blueprint that says "any handler must be able to do these things":

1. **`can_handle(mime_type: &str) -> bool`**: This method checks whether the handler can process a file of the given MIME type. For example, a PDF handler would return `true` for `"application/pdf"` and `false` for other types.

2. **`extract_text(content: &[u8], filename: &str, mime_type: &str) -> Result<String, String>`**: This method takes the raw file content (as bytes) and extracts text from it. It returns either the extracted text on success or an error message on failure.

The trait also specifies that handlers must be `Send + Sync`, which means they can be safely used across multiple threads (required for parallel processing).

### How Handlers Work

Each handler is a self-contained unit that:

1. **Knows which file types it can handle**: Each handler implements `can_handle()` to identify files it can process based on MIME type.

2. **Contains all logic needed for its file type**: Handlers are self-contained, meaning they don't depend on shared helper modules for their specific file type. For example, `TextHandler` contains all the logic for encoding detection and text decoding within itself.

3. **Implements the extraction logic**: Each handler has its own way of extracting text:
   - Text files: Detect encoding and decode the bytes
   - PDF files: Use a PDF parsing library to extract text
   - Word documents: Parse the DOCX XML structure to extract text
   - Excel files: Read spreadsheet cells and convert to text

### Handler Lifecycle

When processing files:

1. **Initialization**: All handlers are created once at startup and stored in a list.

2. **Selection**: For each file, the system asks each handler (in order) if it can handle the file's MIME type using `can_handle()`.

3. **Processing**: The first handler that returns `true` for `can_handle()` is used to extract text from the file.

4. **Result Handling**: The extracted text (or error message) is collected along with file metadata.

## Component Details

### Core Module (`src/core/`)

The core module contains foundational components used throughout the system.

#### FileHandler Trait (`src/core/handler.rs`)

This file defines the `FileHandler` trait, which is the contract all handlers must implement. It's a simple but crucial piece - it defines the interface that allows the system to work with different handler types uniformly.

The trait ensures that:
- Any handler can check if it supports a file type
- Any handler can extract text from supported files
- Handlers can be used safely in parallel processing

### Handlers Module (`src/handlers/`)

The handlers module contains individual handlers for each supported file type. Each handler is independent and self-contained.

#### TextHandler (`src/handlers/text.rs`)

The `TextHandler` processes plain text files and other text-based formats. It handles:

- **MIME Type Detection**: Recognizes text files (starting with `text/`), JSON, XML, JavaScript, TypeScript, and other text-based application types. Also handles CSV and TSV files specifically.

- **Encoding Detection**: Automatically detects the character encoding of text files (like UTF-8, ISO-8859-1, etc.) using the `chardetng` library. This is important because text files can be encoded in different ways, and the wrong encoding will produce garbled text.

- **Text Decoding**: Converts the raw bytes of the file into a readable string using the detected encoding. If decoding fails, it returns an error.

The handler is self-contained, meaning all the logic for MIME type checking and text decoding is within the handler itself, not in a shared module.

#### PdfHandler (`src/handlers/pdf.rs`)

The `PdfHandler` extracts text from PDF documents. It:

- **MIME Type Support**: Handles `application/pdf` files only.

- **Text Extraction**: Uses the `pdf_extract` library to extract text from PDF files loaded in memory.

- **Text Cleaning**: After extraction, it cleans up the text by:
  - Trimming whitespace from each line
  - Removing empty lines
  - Joining lines with newline characters

- **Error Handling**: If PDF extraction fails, it returns a descriptive error message.

#### DocxHandler (`src/handlers/docx.rs`)

The `DocxHandler` extracts text from Microsoft Word documents (DOCX format). It:

- **MIME Type Support**: Handles Word document MIME types:
  - `application/vnd.openxmlformats-officedocument.wordprocessingml.document` (standard DOCX)
  - `application/docx` (alternative MIME type)

- **Document Parsing**: Uses the `docx_rs` library to parse the DOCX file structure. DOCX files are actually ZIP archives containing XML files, and the handler navigates this structure.

- **Text Extraction**: Iterates through the document structure:
  - Finds all paragraphs
  - Extracts text from each paragraph's runs (text segments)
  - Combines all text with newlines between paragraphs

- **Output Formatting**: Trims the final text to remove leading/trailing whitespace.

#### XlsxHandler (`src/handlers/xlsx.rs`)

The `XlsxHandler` extracts text from Microsoft Excel spreadsheets (XLSX format). It:

- **MIME Type Support**: Handles Excel file MIME types:
  - `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet` (standard XLSX)
  - `application/vnd.ms-excel` (legacy Excel format)
  - `application/xlsx` (alternative MIME type)

- **Workbook Parsing**: Uses the `calamine` library to read Excel workbooks from memory.

- **Sheet Processing**: Processes all sheets in the workbook:
  - Iterates through each sheet
  - Adds a header indicating the sheet name
  - Processes each row in the sheet

- **Cell Extraction**: For each row:
  - Converts all cell values to strings
  - Filters out empty cells
  - Joins cells with tab characters (preserving column structure)
  - Adds a newline after each row

- **Output Formatting**: Separates sheets with double newlines and trims the final output.

### Models Module (`src/models/`)

The models module defines the data structures used for input and output.

#### File Data Structures (`src/models/file.rs`)

This file defines three main data structures:

1. **`FileInput`**: Represents an input file to be processed. Contains:
   - `content`: The raw file content as bytes (Buffer)
   - `mime_type`: The MIME type identifying the file format
   - `filename`: The name of the file

2. **`FileMetadata`**: Represents the processed result for a file. Contains:
   - `name`: The original filename
   - `size`: File size in bytes (as a floating-point number)
   - `processing_time_ms`: Time taken to process the file (currently always 0.0)
   - `encoding`: Set to "utf-8" for successfully processed files, "error" for failed extractions, or "application/octet-stream" for unhandled file types
   - `text_content`: The extracted text content

3. **`GroupedFiles`**: Represents files grouped by MIME type. Contains:
   - `mime_type`: The MIME type that groups these files
   - `files`: A list of `FileMetadata` objects for all files of this type

These structures are marked with `#[napi(object)]`, which makes them available to Node.js through the NAPI bindings.

### Main Library (`src/lib.rs`)

The main library file (`src/lib.rs`) is the entry point and orchestration layer. It coordinates all the components to process files.

#### The `process_files` Function

This is the main function exposed to Node.js. It takes a list of `FileInput` objects and returns a list of `GroupedFiles`.

**Initialization Phase**:
1. Creates instances of all handlers (TextHandler, PdfHandler, DocxHandler, XlsxHandler)
2. Wraps them in `Arc` (Atomically Reference Counted) containers, which allows safe sharing across threads
3. Stores them in a list

**Processing Phase** (runs in parallel):
For each file in the input list:
1. **Handler Selection**: Finds the first handler that can process the file by calling `can_handle()` on each handler with the file's MIME type
2. **Text Extraction**: If a handler is found:
   - Calls `extract_text()` on the handler with the file content
   - If successful, sets encoding to "utf-8" for metadata
   - If it fails, stores an error message as the text content and sets encoding to "error"
3. **Metadata Creation**: Creates a `FileMetadata` object with:
   - The filename
   - File size (calculated from content length)
   - Processing time (currently 0.0)
   - Encoding: "utf-8" for successful extractions, "error" for failed extractions, or "application/octet-stream" for unhandled files
   - Extracted text content (or error message)
4. **Grouping**: Adds the metadata to a thread-safe map, grouped by MIME type

**Output Phase**:
1. Converts the grouped map into a list of `GroupedFiles` objects
2. Returns the list

#### Parallel Processing

The system uses `rayon` for parallel processing. The line `files.par_iter()` creates a parallel iterator, which processes multiple files simultaneously across available CPU cores. This significantly speeds up batch processing.

The `DashMap` (a concurrent hash map) is used to safely collect results from parallel threads without data races.

## Processing Flow

Here is the step-by-step flow of how a file is processed:

1. **Input**: A `FileInput` object arrives with file content, MIME type, and filename.

2. **Handler Discovery**: The system iterates through the list of handlers, calling `can_handle()` on each with the file's MIME type.

3. **Handler Selection**: The first handler that returns `true` from `can_handle()` is selected. If no handler matches, the file is marked as unhandled.

4. **Text Extraction**: The selected handler's `extract_text()` method is called with:
   - The raw file content (bytes)
   - The filename
   - The MIME type

5. **Handler-Specific Processing**: Each handler performs its specific extraction:
   - **TextHandler**: Detects encoding, decodes bytes to text
   - **PdfHandler**: Parses PDF structure, extracts and cleans text
   - **DocxHandler**: Parses DOCX XML, extracts text from paragraphs
   - **XlsxHandler**: Reads Excel sheets, converts cells to text

6. **Result Handling**: 
   - If extraction succeeds: The text is stored, and encoding is set to "utf-8" for metadata
   - If extraction fails: An error message is stored as the text content, and encoding is set to "error"

7. **Metadata Assembly**: A `FileMetadata` object is created with all the file information.

8. **Grouping**: The metadata is added to a thread-safe collection, grouped by MIME type.

9. **Output**: After all files are processed, the grouped collection is converted to a list of `GroupedFiles` and returned.

## Key Design Patterns

### Trait-Based Polymorphism

The system uses Rust's trait system to achieve polymorphism (the ability to treat different types uniformly). Instead of using inheritance (like in object-oriented languages), Rust uses traits.

All handlers implement the `FileHandler` trait, which means they can all be stored in the same list and called with the same methods, even though they're different types. This allows the system to:
- Add new file type handlers without changing existing code
- Process different file types with the same code path
- Ensure all handlers follow the same contract

### Self-Contained Handlers

Each handler is self-contained, meaning it doesn't depend on shared helper modules for its specific functionality. This design:
- Makes handlers easier to understand (all related code is in one place)
- Makes handlers easier to test in isolation
- Reduces coupling between components
- Makes it easier to add or remove handlers

For example, `TextHandler` contains its own MIME type checking and text decoding logic, making it completely self-contained.

### Parallel Processing

The system processes files in parallel using Rayon, a Rust library for data parallelism. This means:
- Multiple files can be processed simultaneously
- The system automatically uses all available CPU cores
- Processing time scales with the number of cores
- Thread-safe data structures (`DashMap`, `Arc`) ensure safe concurrent access

The parallel processing is transparent - the code uses `par_iter()` instead of `iter()`, and Rayon handles the rest.

## Summary

The DMS Toolkit uses a handler-based architecture where:

- **Handlers** are self-contained components that know how to process specific file types
- **The FileHandler trait** defines the contract all handlers must follow
- **The main library** orchestrates processing, selecting handlers and collecting results
- **Parallel processing** speeds up batch operations
- **Models** define the data structures for input and output

This design makes the system extensible (easy to add new file types), maintainable (each component has clear responsibilities), and performant (parallel processing and efficient Rust code).

