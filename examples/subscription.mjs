
// npm i @spider-rs/spider-rs
// node subscription.mjs
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://choosealicense.com");

const onPageEvent = (_err, value) => {
  console.log(value);
  console.log(`Links found: ${website.size}`)
};

const id = website.subscribe(onPageEvent);
await website.crawl();
website.unsubscribe(id);