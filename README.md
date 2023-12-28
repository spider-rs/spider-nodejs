# spider-rs

The [spider](https://github.com/spider-rs/spider) project ported to Node.js

## Getting Started

1. `npm i @spider-rs/spider-rs --save`

```ts
import { Website, pageTitle } from "@spider-rs/spider-rs";

const website = new Website("https://rsseau.fr")
  .withHeaders({
    authorization: "somerandomjwt",
  })
  .withBudget({
    "*": 20, // limit max request 20 pages for the website
    "/docs": 10, // limit only 10 pages on the `/docs` paths
  })
  .withBlacklistUrl(["/resume"]) // regex or pattern matching to ignore paths
  .build();

// optional: page event handler
const onPageEvent = (_err, page) => {
  const title = pageTitle(page); // comment out to increase performance if title not needed
  console.info(`Title of ${page.url} is '${title}'`);
  website.pushData({
    status: page.statusCode,
    html: page.content,
    url: page.url,
    title,
  });
};

await website.crawl(onPageEvent);
await website.exportJsonlData("./storage/rsseau.jsonl");
console.log(website.getLinks());
```

Collect the resources for a website.

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://rsseau.fr")
  .withBudget({
    "*": 20,
    "/docs": 10,
  })
  // you can use regex or string matches to ignore paths
  .withBlacklistUrl(["/resume"])
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

const website = new Website("https://rsseau.fr").withChromeIntercept(
  true,
  true,
);

const onPageEvent = (_err, page) => {
  console.log(page);
};

// the third param determines headless chrome usage.
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

View the [benchmarks](./bench/README.md) to see a breakdown between libs and platforms.

Test url: `https://espn.com`

| `libraries`                  | `pages`   | `speed` |
| :--------------------------- | :-------- | :------ |
| **`spider(rust): crawl`**    | `150,387` | `1m`    |
| **`spider(nodejs): crawl`**  | `150,387` | `153s`  |
| **`spider(python): crawl`**  | `150,387` | `186s`  |
| **`scrapy(python): crawl`**  | `49,598`  | `1h`    |
| **`crawlee(nodejs): crawl`** | `18,779`  | `30m`   |

The benches above were ran on a mac m1, spider on linux arm machines performs about 2-10x faster.

## Development

Install the napi cli `npm i @napi-rs/cli --global`.

1. `yarn build:test`
