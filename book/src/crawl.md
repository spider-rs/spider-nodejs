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

## Subscriptions

You can setup many subscriptions to run events when a crawl happens.

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://rsseau.fr");

const onPageEvent = (err, value) => {
  console.log(value);
};

const subscriptionID = website.subscribe(onPageEvent);

await website.crawl(onPageEvent);

website.unsubscribe(subscriptionID);
// this will run instantly as the crawl is in the background
```

## Headless Chrome

Headless Chrome rendering can be done by setting the third param in `crawl` or `scrape` to `true`.
It will attempt to connect to chrome running remotely if the `CHROME_URL` env variable is set with chrome launching as a fallback. Using a remote connection with `CHROME_URL` will
drastically speed up runs.

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://rsseau.fr");

const onPageEvent = (err, value) => {
  console.log(value);
};

// all params are optional. The third param determines headless rendering.
await website.crawl(onPageEvent, false, true);
// make sure to call unsubscribe when finished or else the instance is kept alive when events are setup.
website.unsubscribe();
```
