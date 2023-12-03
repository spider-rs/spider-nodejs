import { CheerioCrawler } from 'crawlee';
import { TEST_URL, iterations } from "../base"

export async function bench() {
    const crawler = new CheerioCrawler({
        async requestHandler({ enqueueLinks, request }) {
          await enqueueLinks();
        }
    });
    
  let duration = 0;

  const run = async () => {
    const startTime = performance.now();
    await crawler.run([TEST_URL]);
    duration += performance.now() - startTime;
  };
  
  const bm = async (cb: () => Promise<void>, i = 0) => {
    await cb();
    if (i < iterations) {
      await bm(cb, i + 1);
    }
  };

  await bm(run);

  console.log(
    JSON.stringify([
      {
        name: "crawlee - OPS/S [SMALL:PAGE]",
        unit: "OPS/S",
        value: 1000 / (duration / iterations),
      },
    ]),
  );
}