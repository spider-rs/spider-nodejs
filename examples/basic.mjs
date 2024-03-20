// npm i @spider-rs/spider-rs
// node basic.mjs
import { Website } from "../index.js";

const url = process.argv[2] || "https://choosealicense.com";

const website = new Website(url).withBudget({ "*": 300, licenses: 0 });

const onPageEvent = (_err, value) => {
  console.log(`Found: ${value.url}`);
};

const startTime = performance.now();

await website.crawl(onPageEvent);

const duration = performance.now() - startTime;

console.log(
  "Finished",
  url,
  "pages found " + website.getLinks().length,
  "elasped duration " + duration + "ms",
);
