# Cron Jobs

Use a cron job that can run any time of day to gather website data.

```ts
import { Website, type NPage } from "@spider-rs/spider-rs";

const website = new Website("https://choosealicense.com")
  .withCron("1/5 * * * * *")
  .build();

const onPageEvent = (err: Error | null, value: NPage) => {
  links.push(value);
};

const handle = await website.runCron(onPageEvent);
```
