# Node.js

We use the node-addon to port the Rust project over with napi to target node.js.

There are some performance drawbacks from the addon, even still the crawls are lightning fast and performant.

## Installation

1. `npm i @spider-rs/spider-rs --save`

## Usage

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