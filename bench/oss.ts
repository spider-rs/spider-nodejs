import { bench } from "./case/spider"
import { bench as benchCrawlee } from "./case/crawlee"

(async () => {
    await bench();
    await benchCrawlee();
})()
