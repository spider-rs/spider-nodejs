
// npm i @spider-rs/spider-rs
// node basic.mjs
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://choosealicense.com");

const onPageEvent = (_err, value) => {
  console.log(value);
};

await website.crawl(onPageEvent);

console.log(website.getLinks());