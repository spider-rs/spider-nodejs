# Crawl

Crawl a website concurrently.

```ts
import { Website } from "@spider-rs/spider-rs";

// pass in the website url
const website = new Website("https://rsseau.fr");

await website.crawl();

// [ "https://rsseau.fr/blog", ...]
console.log(website.getLinks());
```

## Async Event

You can pass in a async function as the first param to the crawl function for realtime updates streamed.

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://rsseau.fr");

const onPageEvent = (err, value) => {
  console.log(value);
};

await website.crawl(onPageEvent);
```

## Background

You can run the request in the background and receive events with the second param set to `true`.

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://rsseau.fr");

const onPageEvent = (err, value) => {
  console.log(value);
};

await website.crawl(onPageEvent, true);
// this will run instantly as the crawl is in the background
```
