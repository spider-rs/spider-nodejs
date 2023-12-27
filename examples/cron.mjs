
// npm i @spider-rs/spider-rs
// node cron.mjs
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://choosealicense.com")
  .withCron("1/5 * * * * *")
  .build();

// get the pages of the website when the cron runs streamed.
const onPageEvent = (_err, value) => {
  console.log(value);
};

const handle = await website.runCron(onPageEvent);
console.log("Starting the Runner for 40 seconds");

setTimeout(async () => {
    await handle.stop()
}, 40000)