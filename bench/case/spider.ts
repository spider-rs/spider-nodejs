import { Website } from '../../index.js'
import { TEST_URL, iterations } from '../base'

export async function bench(url = TEST_URL, size = 'SMALL') {
  const website = new Website(url)

  let duration = 0

  const run = async () => {
    const startTime = performance.now()
    await website.crawl()
    duration += performance.now() - startTime
  }

  const bm = async (cb: () => Promise<void>, i = 0) => {
    await cb()
    if (i < iterations) {
      await bm(cb, i + 1)
    }
  }

  await bm(run)

  console.log(
    JSON.stringify([
      {
        name: `@spider-rs/spider-rs - OPS/S [${size}:PAGE]`,
        unit: 'OPS/S',
        value: 1000 / (duration / iterations),
      },
    ]),
  )
}
