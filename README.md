# spider-rs

The [spider](https://github.com/spider-rs/spider) project ported to nodejs.

## Getting Started

1. `npm i @spider-rs/spider-rs --save`

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://rsseau.fr");

const onPageEvent = (_err, page) => {
  console.log(page);
};

await website.crawl(onPageEvent);
console.log(website.getLinks());
```

Collect the resources for a website.

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://rsseau.fr")
  .withHeaders({
    authorization: "somerandomjwt",
  })
  .withBudget({
    // limit up to 200 pages crawled for the entire website
    "*": 200,
    // limit only 10 pages on the docs page
    "/docs": 10
  })
  .withBlacklistUrl([new RegExp("/books").source, "/resume"])
  .build();

await website.scrape();
console.log(website.getPages());
```

Run the crawls in the background on another thread.

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://rsseau.fr");

const onPageEvent = (_err, page) => {
  console.log(page);
};

await website.crawl(onPageEvent, true);
// runs immediately
```

Use headless Chrome rendering for crawls.

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://rsseau.fr");

const onPageEvent = (_err, page) => {
  console.log(page);
};

await website.crawl(onPageEvent, false, true);
console.log(website.getLinks());
```

Cron jobs can be done with the following.

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://choosealicense.com").withCron(
  "1/5 * * * * *",
);
// sleep function to test cron
const stopCron = (time: number, handle) => {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(handle.stop());
    }, time);
  });
};

const links = [];

const onPageEvent = (err, value) => {
  links.push(value);
};

const handle = await website.runCron(onPageEvent);

// stop the cron in 4 seconds
await stopCron(4000, handle);
```

Use the crawl shortcut to get the page content and url.

```ts
import { crawl } from "@spider-rs/spider-rs";

const { links, pages } = await crawl("https://rsseau.fr");
console.log(pages);
```

## Benchmarks

Spider is about 1,000x (small websites) 10,000x (medium websites), and 100,000x (production grade websites) times faster than the popular crawlee library even with the node port performance hits.

```sh
----------------------
mac Apple M1 Max
10-core CPU
64 GB of RAM memory
1 TB of SSD disk space
-----------------------
```

Test url: `https://choosealicense.com` (small)
32 pages

| `libraries`                           | `speed`               |
| :-------------------------------- | :-------------------- |
| **`spider-rs: crawl 10 samples`** | `286ms`(✅ **1.00x**) |
| **`crawlee: crawl 10 samples`**   | `1.7s` (✅ **1.00x**)   |

Test url: `https://rsseau.fr` (medium)
211 pages

| `libraries`                           | `speed`               |
| :-------------------------------- | :-------------------- |
| **`spider-rs: crawl 10 samples`** | `2.5s` (✅ **1.00x**) |
| **`crawlee: crawl 10 samples`**   | `75s` (✅ **1.00x**)  |

The performance scales the larger the website and if throttling is needed. Linux benchmarks are about 10x faster than macOS for spider-rs.

## Development

Install the napi cli `npm i @napi-rs/cli --global`.

1. `yarn build:test`
