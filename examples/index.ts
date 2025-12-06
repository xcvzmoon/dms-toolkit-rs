import { processFiles, type GroupedFiles } from '../napi';
import { readFileSync } from 'fs';

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

function calculateTotalSize(processedFiles: GroupedFiles[]): number {
  return processedFiles.reduce(
    (total, file) => total + file.files.reduce((sum, metadata) => sum + metadata.size, 0),
    0,
  );
}

function showResult(length: number, sum: number, end: number): void {
  console.log(`Files: ${length}`);
  console.log(`Total: ${formatBytes(sum)}`);
  console.log(`Processed: ${end.toFixed(5)}s`);
}

interface ProcessFileTypeConfig {
  filePath: string;
  mimeType: string;
  filenamePrefix: string;
  filenameExtension: string;
  label: string;
  count?: number;
}

function processFileType(config: ProcessFileTypeConfig): void {
  const { filePath, mimeType, filenamePrefix, filenameExtension, label, count = 10 } = config;

  const buffer = readFileSync(filePath);
  const files = Array.from({ length: count }, (_, index) => ({
    content: buffer,
    mimeType,
    filename: `${filenamePrefix}-${index + 1}.${filenameExtension}`,
  }));

  const start = performance.now();
  const processedFiles = processFiles(files);
  const end = (performance.now() - start) / 1000;

  const totalSize = calculateTotalSize(processedFiles);

  console.log(`\n${label}:`);
  showResult(count, totalSize, end);
}

const fileConfigs: ProcessFileTypeConfig[] = [
  {
    filePath: './examples/files/text.txt',
    mimeType: 'text/plain',
    filenamePrefix: 'text',
    filenameExtension: 'txt',
    label: 'Text Files',
  },
  {
    filePath: './examples/files/pdf.pdf',
    mimeType: 'application/pdf',
    filenamePrefix: 'pdf',
    filenameExtension: 'pdf',
    label: 'PDF Files',
  },
  {
    filePath: './examples/files/word.docx',
    mimeType: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    filenamePrefix: 'word',
    filenameExtension: 'docx',
    label: 'Docx Files',
  },
];

fileConfigs.forEach(processFileType);
