import { processFiles } from '../napi';
import { readFileSync } from 'fs';

function showResult(length: number, sum: number, end: number) {
  function formatBytes(bytes: number): string {
    const KB = 1_000;
    const MB = 1_000_000;
    const GB = 1_000_000_000;

    if (bytes < MB) {
      return `${(bytes / KB).toFixed(2)} KB`;
    } else if (bytes < GB) {
      return `${(bytes / MB).toFixed(2)} MB`;
    } else {
      return `${(bytes / GB).toFixed(2)} GB`;
    }
  }

  console.log(`Files: ${length}`);
  console.log(`Total: ${formatBytes(sum)}`);
  console.log(`Processed: ${end.toFixed(5)}s`);
}

function processTextFiles() {
  const length = 10;
  const textBuffer = readFileSync('./examples/files/text.txt');
  const files = Array.from({ length }).map((_, index) => {
    return {
      content: textBuffer,
      mimeType: 'text/plain',
      filename: `text-${index + 1}.txt`,
    };
  });

  const start = performance.now();
  const processedFiles = processFiles(files);
  const end = (performance.now() - start) / 1000;

  let sum = 0;

  for (const file of processedFiles) {
    for (const metadata of file.files) {
      sum = sum + metadata.size;
    }
  }

  console.log('Text Files:');
  showResult(length, sum, end);
}

function processPdfFiles() {
  const length = 10;
  const pdfBuffer = readFileSync('./examples/files/pdf.pdf');
  const files = Array.from({ length }).map((_, index) => {
    return {
      content: pdfBuffer,
      mimeType: 'application/pdf',
      filename: `pdf-${index + 1}.pdf`,
    };
  });

  const start = performance.now();
  const processedFiles = processFiles(files);
  const end = (performance.now() - start) / 1000;

  let sum = 0;

  for (const file of processedFiles) {
    for (const metadata of file.files) {
      sum = sum + metadata.size;
    }
  }

  console.log('\nPDF Files:');
  showResult(length, sum, end);
}

function processDocxFiles() {
  const length = 10;
  const docxBuffer = readFileSync('./examples/files/word.docx');
  const files = Array.from({ length }).map((_, index) => {
    return {
      content: docxBuffer,
      mimeType: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
      filename: `word-${index + 1}.docx`,
    };
  });

  const start = performance.now();
  const processedFiles = processFiles(files);
  const end = (performance.now() - start) / 1000;

  let sum = 0;

  for (const file of processedFiles) {
    for (const metadata of file.files) {
      sum = sum + metadata.size;
    }
  }

  console.log('\nDocx Files:');
  showResult(length, sum, end);
}

processTextFiles();
processPdfFiles();
processDocxFiles();
