# DMS Toolkit (Rust)

A high-performance file processing toolkit built with Rust and exposed to Node.js via NAPI-RS. Extract text content from various file formats with parallel processing for optimal performance.

> **Note**: This project was created as a learning exercise to explore Rust and its integration with TypeScript. As such, this may not be the best library implementation, and the code may not be optimized or well-structured. Additionally, AI assistance was used for the majority of the development work.

## Features

- 🚀 **High Performance**: Parallel file processing using Rayon
- 📄 **Multiple Formats**: Support for text files, PDFs, DOCX documents, XLSX spreadsheets, CSV files, and images
- 🔍 **Encoding Detection**: Automatic encoding detection for text files
- 🖼️ **OCR Support**: Extract text from images using OCR (Optical Character Recognition)
- 📊 **Grouped Results**: Files are automatically grouped by MIME type
- 🔎 **Similarity Comparison**: Compare extracted text against reference documents with multiple similarity algorithms
- 🦀 **Rust-Powered**: Built with Rust for maximum performance and safety
- 🔌 **Node.js Integration**: Seamless integration with Node.js via NAPI-RS

## Supported File Types

- **Text Files** (`text/plain`, `text/csv`, `text/tsv`): Plain text files with automatic encoding detection
- **PDF Files** (`application/pdf`): Extract text from PDF documents
- **DOCX Files** (`application/vnd.openxmlformats-officedocument.wordprocessingml.document`): Extract text from Microsoft Word documents
- **XLSX Files** (`application/vnd.openxmlformats-officedocument.spreadsheetml.sheet`): Extract text from Excel spreadsheets
- **Image Files** (`image/png`, `image/jpeg`, `image/gif`, `image/bmp`, `image/tiff`, `image/webp`): Extract text from images using OCR

## Tasks

### ✅ Completed

- **Text Files** (`text/plain`): Full support with encoding detection
- **PDF Files** (`application/pdf`): Text extraction implemented
- **DOCX Files** (`application/vnd.openxmlformats-officedocument.wordprocessingml.document`): Text extraction implemented
- **XLSX Files** (`application/vnd.openxmlformats-officedocument.spreadsheetml.sheet`): Excel spreadsheet support
- **CSV Files** (`text/csv`): Comma-separated values file support
- **Image Files**: OCR support for extracting text from images (PNG, JPEG, GIF, BMP, TIFF, WebP)
- **Similarity Comparison**: Compare extracted text against reference documents with multiple algorithms (Jaccard, N-gram, Levenshtein, Hybrid)

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Node.js](https://nodejs.org/) (v16 or higher)
- [pnpm](https://pnpm.io/) (package manager)

### Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd dms-toolkit-rs
```

2. Install dependencies:
```bash
pnpm install
```

3. Build the project:
```bash
pnpm run build
```

## Usage

### Basic Example

```typescript
import { processFiles } from './napi';
import { readFileSync } from 'fs';

const files = [
  {
    content: readFileSync('./path/to/file.pdf'),
    mimeType: 'application/pdf',
    filename: 'document.pdf',
  },
  {
    content: readFileSync('./path/to/file.txt'),
    mimeType: 'text/plain',
    filename: 'document.txt',
  },
];

const processedFiles = processFiles(files);

// Processed files are grouped by MIME type
processedFiles.forEach((group) => {
  console.log(`MIME Type: ${group.mimeType}`);
  group.files.forEach((file) => {
    console.log(`  - ${file.name}: ${file.size} bytes`);
    console.log(`    Encoding: ${file.encoding}`);
    console.log(`    Text Content: ${file.textContent.substring(0, 100)}...`);
  });
});
```

### Similarity Comparison Example

```typescript
import { processAndCompareFiles } from './napi';
import { readFileSync } from 'fs';

const files = [
  {
    content: readFileSync('./path/to/document.pdf'),
    mimeType: 'application/pdf',
    filename: 'document.pdf',
  },
];

const referenceTexts = [
  'This is a reference document for comparison.',
  'Another reference text to compare against.',
];

// Compare files against reference texts
// Options: similarity threshold (default: 30.0), method: 'jaccard' | 'ngram' | 'levenshtein' | 'hybrid' (default: 'hybrid')
const results = processAndCompareFiles(files, referenceTexts, 30.0, 'hybrid');

results.forEach((group) => {
  group.files.forEach((file) => {
    console.log(`File: ${file.name}`);
    console.log(`Similarity Matches: ${file.similarityMatches.length}`);
    file.similarityMatches.forEach((match) => {
      console.log(`  Reference ${match.referenceIndex}: ${match.similarityPercentage.toFixed(2)}%`);
    });
  });
});
```

### Running the Example

The project includes example code that demonstrates processing multiple file types:

```bash
pnpm start
```

Or run in watch mode during development:

```bash
pnpm dev
```

## API Reference

### `processFiles(files: FileInput[]): GroupedFiles[]`

Processes an array of files and returns them grouped by MIME type. Extracts text content from various file formats including text files, PDFs, DOCX documents, XLSX spreadsheets, CSV files, and images (using OCR).

#### Parameters

- `files`: An array of `FileInput` objects

#### FileInput Interface

```typescript
interface FileInput {
  content: Buffer;      // File content as a Buffer
  mimeType: string;     // MIME type of the file
  filename: string;     // Name of the file
}
```

#### Return Value

Returns an array of `GroupedFiles` objects:

```typescript
interface GroupedFiles {
  mimeType: string;     // MIME type of the group
  files: FileMetadata[]; // Array of processed file metadata
}
```

#### FileMetadata Interface

```typescript
interface FileMetadata {
  name: string;              // Original filename
  size: number;              // File size in bytes
  processingTimeMs: number;  // Processing time in milliseconds
  encoding: string;          // Detected encoding (for text files) or "utf-8" for successful extractions, "error" for failures
  textContent: string;       // Extracted text content
}
```

### `processAndCompareFiles(files: FileInput[], referenceTexts: string[], similarityThreshold?: number, similarityMethod?: string): GroupedFilesWithSimilarity[]`

Processes files and compares the extracted text against reference documents using similarity algorithms. Returns files grouped by MIME type with similarity match information.

#### Parameters

- `files`: An array of `FileInput` objects to process
- `referenceTexts`: An array of reference text strings to compare against
- `similarityThreshold`: Optional similarity threshold percentage (default: 30.0). Only matches above this threshold are returned.
- `similarityMethod`: Optional similarity algorithm to use. Options:
  - `"jaccard"`: Fast word-based similarity using Jaccard index
  - `"ngram"`: Character n-gram based similarity (uses 3-grams)
  - `"levenshtein"`: Edit distance based similarity
  - `"hybrid"`: Progressive filtering approach (default) - uses Jaccard for initial filtering, Levenshtein for small texts, and N-gram for larger texts

#### Return Value

Returns an array of `GroupedFilesWithSimilarity` objects:

```typescript
interface GroupedFilesWithSimilarity {
  mimeType: string;     // MIME type of the group
  files: FileMetadataWithSimilarity[]; // Array of processed file metadata with similarity matches
}

interface FileMetadataWithSimilarity {
  name: string;              // Original filename
  size: number;              // File size in bytes
  processingTimeMs: number;  // Processing time in milliseconds
  encoding: string;          // Detected encoding or "utf-8" for successful extractions
  textContent: string;       // Extracted text content
  similarityMatches: SimilarityMatch[]; // Array of similarity matches above threshold
}

interface SimilarityMatch {
  referenceIndex: number;    // Index of the reference text in the input array
  similarityPercentage: number; // Similarity percentage (0-100)
}
```

## Development

### Building

Build for production (release mode):
```bash
pnpm run build
```

Build for development (debug mode):
```bash
pnpm run build:debug
```

### Formatting

Format Rust code:
```bash
pnpm run format
```

### Project Structure

```
dms-toolkit-rs/
├── src/
│   ├── core/           # Core functionality (handlers, text utilities)
│   ├── handlers/       # File type handlers (text, PDF, DOCX)
│   ├── models/         # Data models
│   └── lib.rs          # Main library entry point
├── examples/           # Example usage code
├── napi/              # Generated NAPI bindings
└── Cargo.toml         # Rust dependencies
```

## Performance

The toolkit uses parallel processing to handle multiple files simultaneously, making it ideal for batch processing scenarios. Files are processed concurrently using Rayon's parallel iterators, significantly reducing processing time for large batches.

## Dependencies

### Rust Dependencies

- `napi` / `napi-derive`: Node.js bindings
- `rayon`: Parallel processing
- `dashmap`: Concurrent hash map for grouping
- `chardetng`: Encoding detection for text files
- `pdf-extract`: PDF text extraction
- `docx-rs`: DOCX parsing
- `calamine`: Excel (XLSX) file parsing
- `encoding_rs`: Character encoding support
- `image`: Image format support
- `ocrs`: OCR engine for text extraction from images
- `rten`: Runtime for OCR models
- `strsim`: String similarity algorithms (used internally)

### Node.js Dependencies

- `@napi-rs/cli`: Build tooling
- `tsx`: TypeScript execution
- `@types/node`: TypeScript definitions

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

