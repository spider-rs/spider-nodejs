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
