import { processFiles, type GroupedFiles, processAndCompareFiles, type GroupedFilesWithSimilarity } from '../napi';
import { readFileSync } from 'fs';

type ProcessFileTypeConfig = {
  filePath: string;
  mimeType: string;
  filenamePrefix: string;
  filenameExtension: string;
  label: string;
  count?: number;
};

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
  return processedFiles.reduce((total, file) => {
    return total + file.files.reduce((sum, metadata) => sum + metadata.size, 0);
  }, 0);
}

function calculateTotalSizeForCompared(processedFiles: GroupedFilesWithSimilarity[]): number {
  return processedFiles.reduce((total, file) => {
    return total + file.files.reduce((sum, metadata) => sum + metadata.size, 0);
  }, 0);
}

function showResult(length: number, sum: number, end: number): void {
  console.log(`  📊 Files: ${length}`);
  console.log(`  💾 Total: ${formatBytes(sum)}`);
  console.log(`  ⏱️ Processed: ${end.toFixed(5)}s`);
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

  console.log(`📄 ${label}`);
  showResult(count, totalSize, end);

  processedFiles.forEach((group) => {
    if (group.files.length > 0) {
      const firstFile = group.files[0];
      console.log(`  📝 Content Preview (${group.mimeType}):`);
      console.log(`       ${firstFile.textContent.substring(0, 25)}${firstFile.textContent.length > 25 ? '...' : ''}\n`);
    }
  });
}

const loremIpsumTexts: string[] = [
  'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.',
  'Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.',
  'Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.',
  'Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet.',
  'At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident.',
];

function processCompareFileType(config: ProcessFileTypeConfig): void {
  const { filePath, mimeType, filenamePrefix, filenameExtension, label, count = 10 } = config;

  const buffer = readFileSync(filePath);
  const files = Array.from({ length: count }, (_, index) => ({
    content: buffer,
    mimeType,
    filename: `${filenamePrefix}-${index + 1}.${filenameExtension}`,
  }));

  const start = performance.now();
  const processedFiles = processAndCompareFiles(files, loremIpsumTexts, 1.0, 'hybrid');
  const end = (performance.now() - start) / 1000;

  const totalSize = calculateTotalSizeForCompared(processedFiles);

  console.log(`🔍 ${label} - Similarity Comparison`);
  showResult(count, totalSize, end);
  console.log(`  🔎 Comparison texts: ${loremIpsumTexts.length} lorem ipsum variations`);
  console.log(`  📏 Similarity threshold: 10.0%`);

  processedFiles.forEach((group) => {
    if (group.files.length > 0) {
      const firstFile = group.files[0];
      console.log(`\n${'─'.repeat(70)}`);
      console.log(`📁 File: ${firstFile.name}`);
      console.log(`   Type: ${group.mimeType}`);
      console.log(`   Size: ${formatBytes(firstFile.size)}`);

      if (firstFile.textContent.trim().length > 0) {
        console.log(`\n   📝 Extracted Text Preview:`);
        const preview = firstFile.textContent.substring(0, 25).trim();
        console.log(`      "${preview}${firstFile.textContent.length > 25 ? '...' : ''}"`);
      }

      if (firstFile.similarityMatches.length > 0) {
        console.log(`\n   ✅ Found ${firstFile.similarityMatches.length} similarity match(es):`);
        console.log(`   ${'─'.repeat(68)}`);
        firstFile.similarityMatches.forEach((similarity, idx) => {
          const percentage = similarity.similarityPercentage;
          const barLength = Math.min(30, Math.floor(percentage / 3.33));
          const bar = '█'.repeat(barLength) + '░'.repeat(30 - barLength);
          const emoji = percentage >= 80 ? '🟢' : percentage >= 60 ? '🟡' : '🟠';

          const comparisonText = loremIpsumTexts[similarity.referenceIndex] || 'Unknown';
          console.log(`\n   ${emoji} Match #${idx + 1}: Comparison Text ${similarity.referenceIndex + 1}`);
          console.log(`      Similarity: ${percentage.toFixed(2)}%  [${bar}]`);
          console.log(`      Comparison Text:`);
          const compText = comparisonText.substring(0, 120).trim();
          console.log(`      "${compText}${comparisonText.length > 120 ? '...' : ''}"`);
        });
      } else {
        console.log(`\n   ❌ No similarity matches found`);
        console.log(`      (All comparisons below 1.0% threshold)`);
      }
    }
  });
  console.log(`${'═'.repeat(70)}\n`);
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
  {
    filePath: './examples/files/spreadsheet.xlsx',
    mimeType: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
    filenamePrefix: 'excel',
    filenameExtension: 'xlsx',
    label: 'Excel Files',
  },
  {
    filePath: './examples/files/csv.csv',
    mimeType: 'text/csv',
    filenamePrefix: 'csv',
    filenameExtension: 'csv',
    label: 'CSV Files',
  },
  {
    filePath: './examples/files/image.png',
    mimeType: 'image/png',
    filenamePrefix: 'image',
    filenameExtension: 'png',
    label: 'Image Files',
  },
];

console.log('\n' + '╔' + '═'.repeat(78) + '╗');
console.log('║' + ' '.repeat(20) + 'FILE PROCESSING TEST' + ' '.repeat(38) + '║');
console.log('╚' + '═'.repeat(78) + '╝');

fileConfigs.forEach(processFileType);

console.log('\n\n' + '╔' + '═'.repeat(78) + '╗');
console.log('║' + ' '.repeat(15) + 'FILE COMPARISON WITH LOREM IPSUM TEXTS' + ' '.repeat(25) + '║');
console.log('╚' + '═'.repeat(78) + '╝');

fileConfigs.forEach(processCompareFileType);
