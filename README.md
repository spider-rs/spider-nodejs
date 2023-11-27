# spider-rs

The [spider](https://github.com/spider-rs/spider) project ported to nodejs via napi.

## Getting Started

1. `npm i @spider-rs/spider-rs --save`

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://rsseau.fr");

const onPageEvent = (_err, value) => {
  console.log(value);
};

await website.crawl(onPageEvent);
console.log(website.getLinks());
```

Collect the resources for a website. View [config](https://docs.rs/spider/latest/spider/website/struct.Website.html) for options, when using convert the method to camelCase.

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://rsseau.fr")
  .withHeaders({
    authorization: "somerandomjwt",
  })
  .withBudget({
    // limit up to 200 pages crawled for the entire website
    "*": 200,
  })
  .withBlacklistUrl([new RegExp("/books").source, "/resume"])
  .build();

await website.scrape();
console.log(website.getPages());
```

Use the crawl shortcut to get the page content and url.

```ts
import { crawl } from "@spider-rs/spider-rs";

const { links, pages } = new crawl("https://rsseau.fr");
console.log(pages);
```

## Benchmarks

```sh
----------------------
mac Apple M1 Max
10-core CPU
64 GB of RAM memory
1 TB of SSD disk space
-----------------------

Test url: `https://choosealicense.com` (small)

32 pages

```

|                                   | `libraries`           |
| :-------------------------------- | :-------------------- |
| **`spider-rs: crawl 10 samples`** | `390ms`(✅ **1.00x**) |
| **`crawlee: crawl 10 samples`**   | `1s` (✅ **1.00x**)   |

---

Test url: `https://rsseau.fr` (medium)

211 pages

```

|                                   | `libraries`          |
| :-------------------------------- | :------------------- |
| **`spider-rs: crawl 10 samples`** | `4s` (✅ **1.00x**)  |
| **`crawlee: crawl 10 samples`**   | `75s` (✅ **1.00x**) |


The performance scales the larger the website and if throttling is needed.

## Development

Install the napi cli `npm i @napi-rs/cli --global`.

1. `yarn build:test`
```
