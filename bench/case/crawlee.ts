// @ts-ignore
import { CheerioCrawler } from "crawlee";
import { TEST_URL, iterations } from "../base";

export async function bench(url = TEST_URL, size = "SMALL") {
  // @ts-ignore
  const crawler = new CheerioCrawler({
    // @ts-ignore
    async requestHandler({ enqueueLinks }) {
      // @ts-ignore
      await enqueueLinks();
    },
  });

  let duration = 0;

  const run = async () => {
    const startTime = performance.now();
    // @ts-ignore
    await crawler.run([url]);
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
        name: `crawlee - OPS/S [${size}:PAGE]`,
        unit: "OPS/S",
        value: 1000 / (duration / iterations),
      },
    ]),
  );
}
