# Cron Jobs

Use a cron job that can run any time of day to gather website data.

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://choosealicense.com")
  .withCron("1/5 * * * * *")
  .build();

// get the pages of the website when the cron runs streamed.
const onPageEvent = (err, value) => {
  console.log(value);
};

const handle = await website.runCron(onPageEvent);
```
