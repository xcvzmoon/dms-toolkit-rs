/**
 * M1/8Gb Testing Environment
 * This is without workers
 *
 * Without Content Extraction: > 1_000_000 files
 * With Content Extraction (text files): < 300_000 files
 */

import { processFiles, extractTextContent } from '../napi';

const LENGTH = 10;

const files = Array(LENGTH)
  .fill(null)
  .map((_, i) => ({
    content: Buffer.from(`
      File - ${i + 1}

      Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut accumsan posuere elementum. Curabitur et aliquet risus, eget ultrices nunc. Praesent auctor, lectus id cursus facilisis, ex ipsum vestibulum eros, in lacinia nulla orci quis tellus. Pellentesque luctus ac elit sed varius. Nullam eleifend nisi lectus, in pellentesque metus condimentum ut. Quisque rhoncus porta magna sit amet tempus. Fusce malesuada nunc ut sem pharetra pharetra. Sed euismod, sem sed ultricies tincidunt, lectus leo interdum metus, id volutpat arcu eros at arcu. Quisque non orci eu nisi cursus placerat ac ac ante. Duis quis luctus nunc. Morbi fringilla, libero in fringilla maximus, ante velit feugiat dui, in congue lacus leo quis metus.

      Aenean bibendum lacinia ipsum, sit amet commodo libero luctus et. Mauris sodales fermentum augue, vitae ultricies metus maximus id. Suspendisse dictum turpis quis ipsum venenatis feugiat. Maecenas mattis vulputate nibh, eget efficitur neque ullamcorper facilisis. Integer viverra sollicitudin nulla. Phasellus consectetur ante at nulla fringilla, quis vulputate nibh euismod. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Suspendisse potenti. Morbi nec aliquam mauris, vel tempus risus. Aliquam dui urna, faucibus eu est vitae, scelerisque dignissim augue. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc imperdiet augue metus, vitae semper mi ullamcorper id. Morbi id aliquam quam.

      Quisque ut justo et nulla congue porta id a diam. Cras aliquam dolor elit, consectetur ullamcorper odio ullamcorper vel. Integer euismod consequat volutpat. Donec tempus consectetur vulputate. Vestibulum tempor ultricies velit, eu consectetur tortor mattis id. Morbi vitae ligula malesuada, aliquet quam at, ultricies nisl. Quisque tortor magna, venenatis quis iaculis eget, aliquam nec lacus. Ut porttitor lacus enim, ac dictum diam pretium eget. Vestibulum dapibus et turpis id aliquet. Vivamus congue tellus velit, sed elementum mi ornare et. Donec at turpis tristique, eleifend eros vitae, fermentum arcu. Duis efficitur luctus ipsum, in rhoncus elit rhoncus vel. Nunc a consequat mauris.

      Curabitur accumsan molestie sapien, et laoreet mauris aliquam eu. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean id lectus ornare metus posuere cursus sed ac sapien. Donec ac vehicula odio. Nam fringilla a mi vel venenatis. Mauris tristique nisl vel interdum posuere. Nam pellentesque lectus eleifend nisi vulputate volutpat sit amet in libero. Nullam at tellus vitae nisi mollis venenatis. Pellentesque dictum elit non venenatis mollis. Vivamus faucibus a lectus a mattis. Quisque dictum, nisi at imperdiet mollis, ipsum nunc maximus mi, vel pretium leo nisl vel enim. Nullam tincidunt tincidunt massa vel tincidunt. Maecenas gravida sem et auctor fringilla. Aliquam non lectus libero. Sed fermentum a arcu at fringilla. Phasellus ac dapibus augue, sit amet sagittis metus.

      Nulla vel elit sagittis, euismod mauris eu, venenatis arcu. Sed quis ante cursus, vehicula purus sed, imperdiet justo. Cras posuere eu ante a facilisis. Vestibulum dolor velit, efficitur quis gravida sed, scelerisque vitae ipsum. Sed laoreet congue metus, quis vestibulum nulla euismod pulvinar. Integer tincidunt lectus et dignissim fermentum. Curabitur mattis lacus et sagittis finibus. Nam metus augue, cursus sed auctor nec, tempus at erat.

      Vestibulum eu sapien lacus. Pellentesque aliquam nisl sit amet augue consectetur suscipit. Quisque ultrices suscipit auctor. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Donec id faucibus felis, ac elementum enim. Suspendisse fermentum ac augue vel dictum. Etiam sit amet ultricies nisl, eget gravida purus. Proin vel tortor enim. Curabitur egestas urna felis, sit amet convallis ante venenatis in. Phasellus ultrices tellus eget tellus consequat, a imperdiet ex accumsan. Mauris et blandit libero.

      Mauris et eros laoreet, gravida mauris nec, vestibulum nisi. Nam vehicula leo dapibus urna consequat luctus. Sed eu massa dui. Donec ut pretium arcu. Pellentesque ut luctus sem. Aenean sodales semper lectus. Proin mollis ante ante, quis bibendum ante feugiat convallis. Proin id enim ac risus bibendum elementum non laoreet est. Cras pretium at felis et vehicula. Nunc posuere tortor in tellus consectetur, id dapibus arcu commodo. Curabitur sit amet cursus nulla, id auctor sem. Interdum et malesuada fames ac ante ipsum primis in faucibus.

      Vivamus feugiat ligula in dui hendrerit, a ultricies mauris consequat. Praesent porttitor efficitur augue varius consectetur. Morbi bibendum sagittis diam, sit amet vestibulum est facilisis pulvinar. Duis eget urna nec massa venenatis semper ac at lorem. Curabitur nisi nunc, congue ac massa sit amet, elementum posuere arcu. Aliquam non quam eu quam ornare auctor. Phasellus bibendum justo rhoncus libero condimentum consectetur. Pellentesque finibus risus in ex mattis ultricies. Suspendisse eu felis eu odio auctor finibus porta et eros. Proin ac diam luctus eros laoreet placerat vitae sed turpis. Donec quam dui, consectetur et turpis sed, molestie commodo urna. Ut vestibulum tincidunt augue in condimentum. Donec gravida faucibus elit, in pretium sapien blandit vitae. Interdum et malesuada fames ac ante ipsum primis in faucibus. Duis iaculis luctus velit sit amet consectetur.

      Cras ut metus non massa congue hendrerit. Mauris sed magna enim. Curabitur scelerisque dui felis, sed venenatis eros volutpat id. Curabitur venenatis ac nibh et commodo. Praesent diam velit, vehicula sit amet sagittis ornare, aliquet nec sem. Pellentesque dolor mauris, tincidunt vel magna ut, accumsan commodo est. Aliquam erat volutpat. Sed ac felis vel nisi dignissim accumsan.

      Integer sed sagittis eros. In pharetra elit lorem, quis consectetur orci viverra mattis. Morbi pharetra libero id ullamcorper fermentum. Aenean vestibulum lorem quis porta aliquet. Nam luctus mi leo, non tincidunt lacus tristique non. Mauris fringilla lectus sed ex posuere volutpat. Mauris vitae sodales ex. Duis pulvinar vulputate purus, id sollicitudin odio tempus nec. Praesent vitae odio facilisis, rutrum libero non, posuere elit. Nulla non eros sem. Etiam ac viverra turpis. Donec lectus lacus, auctor vel libero vel, tempus sodales nulla. Integer non justo non felis convallis porttitor. Mauris purus odio, elementum sed ex eget, tempor vehicula felis. Aenean a nibh a tellus maximus vestibulum sed congue quam.

      `),
    mimeType: 'text/plain',
    filename: `${i + 1}-text-test-file.txt`,
  }));

const start = performance.now();
const processedFiles = processFiles(files);
const end = (performance.now() - start) / 1000;

let sum = 0;

for (const file of processedFiles) {
  for (const metadata of file.files) {
    sum = sum + metadata.size;
  }
}

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

console.log(`Files: ${LENGTH}`);
console.log(`Total: ${formatBytes(sum)}`);
console.log(`Processed: ${end.toFixed(5)}s`);
console.log(extractTextContent(files[0].content, files[0].mimeType));
