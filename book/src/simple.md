# A simple example

We use the node-addon to port the Rust project over with napi to target node.js.

There are some performance drawbacks from the addon, even still the crawls are lightning fast and efficient.

## Usage

The examples below can help get started with spider.

### Basic

A basic example.

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://choosealicense.com");

await website.crawl();
console.log(website.getLinks());
```

### Events

You can pass a function that could be async as param to `crawl` and `scrape`.

```ts
import { Website, type NPage } from "@spider-rs/spider-rs";

const website = new Website("https://choosealicense.com");

const links: NPage[] = [];

const onPageEvent = async (err: Error | null, page: NPage) => {
  links.push(page);
};

await website.crawl(onPageEvent);
console.log(website.getLinks());
```

### Selector

The `title` method allows you to extract the title of the page.

```ts
import { Website, pageTitle } from "@spider-rs/spider-rs";

const website = new Website("https://choosealicense.com");

const links = [];

const onPageEvent = async (err, page) => {
  links.push({ title: pageTitle(page), url: page.url });
};

// params in order event, background, and headless chrome
await website.crawl(onPageEvent);
```

## Shortcut

You can use the `crawl` shortcut method to collect contents quickly without configuration.

```ts
import { crawl } from "@spider-rs/spider-rs";

const { links, pages } = await crawl("https://choosealicense.com");

console.log([links, pages]);
```
