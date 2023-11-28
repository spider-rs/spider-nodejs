# A simple example

We use the node-addon to port the Rust project over with napi to target node.js.

There are some performance drawbacks from the addon, even still the crawls are lightning fast and performant.

## Usage

The examples below can help get started with spider.

### Basic

A basic example.

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

await website.crawl();
console.log(website.getLinks());
```

### Events

You can pass a function that could be async as param to `crawl` and `scrape`.

```ts
import { Website, type NPage } from "@spider-rs/spider-rs";

const website = new Website("https://rsseau.fr");

const links: NPage[] = [];

const onPageEvent = (err: Error | null, value: NPage) => {
  links.push(value);
};

await website.crawl(onPageEvent);
console.log(website.getLinks());
```
