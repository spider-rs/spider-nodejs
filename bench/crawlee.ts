import { TEST_URL_MEDIUM, TEST_URL_LARGE, BenchSizes } from "./base";
import { bench } from "./case/crawlee";

// small
bench();
// small/medium
bench(TEST_URL_MEDIUM, BenchSizes.MEDIUM);
// large 150k pages plus
if (process.env.BENCH_LARGE) {
    bench(TEST_URL_LARGE, BenchSizes.LARGE)
}