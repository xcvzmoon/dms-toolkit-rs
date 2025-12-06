# DMS Toolkit (Rust)

A high-performance file processing toolkit built with Rust and exposed to Node.js via NAPI-RS. Extract text content from various file formats with parallel processing for optimal performance.

## Features

- 🚀 **High Performance**: Parallel file processing using Rayon
- 📄 **Multiple Formats**: Support for text files, PDFs, and DOCX documents
- 🔍 **Encoding Detection**: Automatic encoding detection for text files
- 📊 **Grouped Results**: Files are automatically grouped by MIME type
- 🦀 **Rust-Powered**: Built with Rust for maximum performance and safety
- 🔌 **Node.js Integration**: Seamless integration with Node.js via NAPI-RS

## Supported File Types

- **Text Files** (`text/plain`): Plain text files with encoding detection
- **PDF Files** (`application/pdf`): Extract text from PDF documents
- **DOCX Files** (`application/vnd.openxmlformats-officedocument.wordprocessingml.document`): Extract text from Microsoft Word documents

## Tasks

### ✅ Completed

- **Text Files** (`text/plain`): Full support with encoding detection
- **PDF Files** (`application/pdf`): Text extraction implemented
- **DOCX Files** (`application/vnd.openxmlformats-officedocument.wordprocessingml.document`): Text extraction implemented

### 🚧 Planned / Not Yet Implemented

- **XLSX Files** (`application/vnd.openxmlformats-officedocument.spreadsheetml.sheet`): Excel spreadsheet support
- **CSV Files** (`text/csv`): Comma-separated values file support
- **Image Files**: Support for extracting text from images (OCR) for formats like:
  - PNG (`image/png`)
  - JPEG (`image/jpeg`)
  - GIF (`image/gif`)
  - BMP (`image/bmp`)

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

Processes an array of files and returns them grouped by MIME type.

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
  encoding: string;          // Detected encoding (for text files)
  textContent: string;       // Extracted text content
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
- `chardetng`: Encoding detection
- `pdf-extract`: PDF text extraction
- `docx-rs`: DOCX parsing
- `encoding_rs`: Character encoding support

### Node.js Dependencies

- `@napi-rs/cli`: Build tooling
- `tsx`: TypeScript execution
- `@types/node`: TypeScript definitions

## License

ISC

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

