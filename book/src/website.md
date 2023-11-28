# Website

The Website class is the foundations to the spider.

## Builder pattern

We use the builder pattern to configure the website for crawling.

\*note: Replace `https://choosealicense.com` from the examples below with your website target URL.

All of the examples use typescript by default.

```ts
import { Website } from "@spider-rs/spider-rs";

const website = new Website("https://choosealicense.com");
```

### Custom Headers

Add custom HTTP headers to use when crawling/scraping.

```ts
const website = new Website("https://choosealicense.com")
  .withHeaders({
    authorization: "somerandomjwt",
  })
  .build();
```

### Blacklist

Prevent crawling a set path, url, or pattern with Regex.

```ts
const website = new Website("https://choosealicense.com")
  .withBlacklistUrl(["/blog", new RegExp("/books").source, "/resume"])
  .build();
```

### Crons

Setup a cron job that can run at any time in the background using cron-syntax.

```ts
const website = new Website("https://choosealicense.com")
  .withCron("1/5 * * * * *")
  .build();
```

View the [cron](./cron-job.md) section for details how to use the cron.
