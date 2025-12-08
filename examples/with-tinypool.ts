import { readFileSync } from 'fs';

import Tinypool from 'tinypool';

const pool = new Tinypool({
  filename: new URL('./worker.mjs', import.meta.url).href,
  minThreads: 2,
  maxThreads: 8,
  idleTimeout: 30000,
});

const referenceTexts: string[] = [
  `Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc egestas ornare enim, eget mollis eros
maximus in. Nullam at cursus leo, ac hendrerit nisl. Maecenas pulvinar bibendum nibh ac laoreet. In
et mauris pharetra, pellentesque magna ac, eleifend felis. Aliquam iaculis, nunc sed mattis dapibus,
turpis lacus iaculis leo, aliquet interdum ligula turpis nec neque. Aenean pulvinar gravida consequat.
Nam rutrum ipsum eu justo maximus feugiat. Ut dictum ut dui in pretium. Praesent aliquam ante
ipsum, id tempus dui sodales id. Donec et varius risus. Nulla aliquam feugiat ultricies. Praesent
varius sem leo, non suscipit felis posuere et. Etiam fringilla dui quis erat gravida tincidunt. Integer
varius non lacus in scelerisque. Quisque scelerisque pellentesque arcu laoreet varius. Vestibulum
eleifend arcu sed erat pretium, vitae lobortis magna blandit.`,
  `Duis sagittis suscipit condimentum. Donec urna libero, congue consequat placerat euismod, dictum
ac turpis. Donec vitae neque at mauris hendrerit elementum ut quis ante. Nam sed scelerisque
augue. Nullam sem lorem, fermentum et urna vitae, bibendum elementum lacus. Nunc blandit iaculis
quam auctor varius. Maecenas et facilisis massa. Curabitur condimentum vitae neque quis aliquet.
Etiam ac eros eleifend nisl volutpat tincidunt ut ac dolor.`,
  `Cras nisl odio, rhoncus a hendrerit ac, vulputate sed nulla. Integer tristique turpis leo, vitae
elementum eros volutpat et. Sed ac sapien nec urna sagittis dignissim. Morbi blandit lorem et metus
dictum ornare. Aliquam et dapibus erat, id sagittis orci. Suspendisse tincidunt efficitur ullamcorper. Ut
vitae dolor leo. Ut sapien purus, varius sed elit vel, finibus sodales nulla. Duis molestie dolor ut erat
facilisis vulputate. Donec sit amet tristique purus, quis pulvinar justo. Aliquam convallis sem nec
accumsan bibendum. Donec sed facilisis nisl. In consectetur orci eget dictum consequat.`,
  `Quisque consequat lorem eget rhoncus imperdiet. Aliquam eget ipsum consectetur, malesuada odio
vel, ullamcorper libero. Proin semper bibendum efficitur. Duis semper leo nec sodales tempor. Proin
at dignissim ante, non laoreet lacus. Orci varius natoque penatibus et magnis dis parturient montes,
nascetur ridiculus mus. Aliquam blandit et augue sit amet ultrices. Quisque non quam purus.`,
  `Sed condimentum dui vel metus facilisis, vitae volutpat sapien sagittis. Aenean eget finibus justo. Nam
venenatis blandit elit, eget rutrum orci aliquet sit amet. Aenean id viverra nunc. Aenean imperdiet
ullamcorper egestas. Phasellus tincidunt ullamcorper posuere. Donec vitae sem sed purus facilisis
viverra. Donec vehicula ex et erat dignissim posuere. Fusce a aliquam leo. Praesent pretium mi sed
fringilla sodales. Pellentesque tempus lacus enim, ac efficitur ante pulvinar nec. Pellentesque
sodales arcu sit amet bibendum vehicula. Donec ac elit arcu. Nulla facilisis gravida ante, ut
vestibulum mauris rhoncus vitae. Integer vel lobortis sapien, eu tincidunt dui.`,
];

const buffer = readFileSync('./examples/files/text-v2.txt');
const files = Array.from({ length: 10 }, (_, index) => ({
  content: buffer,
  mimeType: 'text/plain',
  filename: `text-${index + 1}.txt`,
}));

(async () => {
  const start = performance.now();
  const result = await pool.run({ files, referenceTexts });
  const end = performance.now();

  for (const group of result) {
    for (const file of group.files) {
      console.log(file.name);
      console.log(file.similarityMatches);
    }
  }

  console.log(`Time taken: ${end - start}ms`);
})();
