# Page

A single page on a website, useful if you just need one or the root url.

## New Page

Get a new page with content.

The first param is the url, followed by if subdomains should be included, and last to include TLD's in links.

Calling `page.fetch` is needed to get the content.

```ts
import { Page } from "@spider-rs/spider-rs";

const page = new Page("https://choosealicense.com", false, false);
await page.fetch();
```

## Page Links

get all the links related to a page.

```ts

const page = new Page("https://choosealicense.com", false, false);
await page.fetch();
const links = await page.getLinks();
console.log(links);
```

## Page Html

Get the markup for the page or HTML.

```ts

const page = new Page("https://choosealicense.com", false, false);
await page.fetch();
const html = page.getHtml();
console.log(html);
```

## Page Bytes

Get the raw bytes of a page to store the files in a database.

```ts

const page = new Page("https://choosealicense.com", false, false);
await page.fetch();
const bytes = page.getBytes();
console.log(bytes);
```