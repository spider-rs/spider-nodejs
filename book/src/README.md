# Introduction

Spider-RS is the fastest web crawler and indexer written in Rust ported to Node.js.

Spider powers some big tools and helps bring the crawling aspect to almost no downtime with the correct setup, view the [spider](https://github.com/spider-rs/spider) project to learn more.

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://choosealicense.com");

await website.crawl();

console.log(website.getLinks());
```
