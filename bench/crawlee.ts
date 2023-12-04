import { TEST_URL_MEDIUM, BenchSizes } from "./base";
import { bench } from "./case/crawlee";

// small
bench();
// small/medium
bench(TEST_URL_MEDIUM, BenchSizes.MEDIUM)
