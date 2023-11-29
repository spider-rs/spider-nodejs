# Scrape

Scape a website and collect the resource data.

```ts
import { Website } from "@spider-rs/spider-rs";

// pass in the website url
const website = new Website("https://rsseau.fr");

await website.scrape();

// [ { url: "https://rsseau.fr/blog", html: "<html>...</html>"}, ...]
console.log(website.getPages());
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
await website.scrape(onPageEvent, false, true);
```
