import { bench } from './case/spider'
import { bench as benchCrawlee } from './case/crawlee'
import { TEST_URL_MEDIUM, BenchSizes } from './base'
;(async () => {
  await bench()
  await bench(TEST_URL_MEDIUM, BenchSizes.MEDIUM)
  await benchCrawlee()
  await benchCrawlee(TEST_URL_MEDIUM, BenchSizes.MEDIUM)
})()
