import { Website } from "../index.js";

const iterations = process.env.BENCH_COUNT
  ? parseInt(process.env.BENCH_COUNT, 10)
  : 20;

const TEST_URL = "https://choosealicense.com";

async function bench() {
  const website = new Website(TEST_URL);

  let duration = 0;

  const run = async () => {
    const startTime = performance.now();
    await website.crawl();
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
        name: "@spider-rs/spider-rs - OPS/S [SMALL:PAGE]",
        unit: "OPS/S",
        value: 1000 / (duration / iterations),
      },
    ]),
  );
}

bench();
