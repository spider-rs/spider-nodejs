# Website

The Website class is the foundations to the spider.

## Builder pattern

We use the builder pattern to configure the website for crawling.

\*note: Replace `https://choosealicense.com` from the examples below with your website target URL.

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

### Budget

Add a crawl budget that prevents crawling `x` amount of pages.

```ts
const website = new Website("https://choosealicense.com")
  .withBudget({
    "*": 1,
  })
  .build();
```

### Subdomains

Include subdomains in request.

```ts
const website = new Website("https://choosealicense.com")
  .withSubdomains(true)
  .build();
```

### TLD

Include TLDs in request.

```ts
const website = new Website("https://choosealicense.com")
  .withTlds(true)
  .build();
```

### External Domains

Add external domains to include with the website.

```ts
const website = new Website("https://choosealicense.com")
  .withExternalDomains(["https://www.myotherdomain.com"])
  .build();
```

### Proxy

Use a proxy to crawl a website.

```ts
const website = new Website("https://choosealicense.com")
  .withProxies(["https://www.myproxy.com"])
  .build();
```

### Delays

Add delays between pages. Defaults to none.

```ts
const website = new Website("https://choosealicense.com")
  .withDelays(200)
  .build();
```

### User-Agent

Use a custom User-Agent.

```ts
const website = new Website("https://choosealicense.com")
  .withUserAgent("mybot/v1")
  .build();
```

### Request Timeout

Add a request timeout per page in miliseconds. Example shows 30 seconds.

```ts
const website = new Website("https://choosealicense.com")
  .withRequestTimeout(30000)
  .build();
```

### Respect Robots

Respect the robots.txt file.

```ts
const website = new Website("https://choosealicense.com")
  .withRespectRobotsTxt(true)
  .build();
```

### Http2 Prior Knowledge

Use http2 to connect if you know the website servers supports this.

```ts
const website = new Website("https://choosealicense.com")
  .withHttp2PriorKnowledge(true)
  .build();
```

### Chrome Network Interception

Enable Network interception when using chrome to speed up request.

```ts
const website = new Website("https://choosealicense.com")
  .withChromeIntercept(true, true)
  .build();
```

### Redirect Limit

Set the redirect limit for request.

```ts
const website = new Website("https://choosealicense.com")
  .withRedirectLimit(2)
  .build();
```

### Depth Limit

Set the depth limit for the amount of forward pages.

```ts
const website = new Website("https://choosealicense.com")
  .withDepth(3)
  .build();
```

### Cache

Enable HTTP caching, this useful when using the spider on a server.

```ts
const website = new Website("https://choosealicense.com")
  .withCaching(true)
  .build();
```

### Redirect Policy

Set the redirect policy for request, either strict or loose(default). Strict only allows redirects that match the domain.

```ts
const website = new Website("https://choosealicense.com")
  .withRedirectPolicy(true)
  .build();
```

## Chaining

You can chain all of the configs together for simple configuration.

```ts
const website = new Website("https://choosealicense.com")
  .withSubdomains(true)
  .withTlds(true)
  .withUserAgent("mybot/v1")
  .withRespectRobotsTxt(true)
  .build();
```

## Raw Content

Set the second param of the website constructor to `true` to return content without UTF-8.
This will return `rawContent` and leave `content` when using subscriptions or the Page Object.

```ts
const rawContent = true;
const website = new Website("https://choosealicense.com", rawContent);
await website.scrape();
```

## Clearing Crawl Data

Use `website.clear` to remove the links visited and page data or `website.drainLinks` to drain the links visited.

```ts
const website = new Website("https://choosealicense.com");
await website.crawl();
// links found ["https://...", "..."]
console.log(website.getLinks());
website.clear();
// links will be empty
console.log(website.getLinks());
```

## Storing and Exporting Data

Collecting data to store can be done with `website.pushData()` and `website.exportJsonlData()`.

```ts
const website = new Website("https://choosealicense.com");

const onPageEvent = (_err, page) => {
  website.pushData(page);
};

await website.crawl(onPageEvent);

// uncomment to read the data.
// console.log(website.readData());

// we only have one export method atm. Optional file path. All data by default goes to storage
await website.exportJsonlData("./storage/test.jsonl");
```

## Stop crawl

To stop a crawl you can use `website.stopCrawl(id)`, pass in the crawl id to stop a run or leave empty for all crawls to stop.

```ts
const website = new Website("https://choosealicense.com");

const onPageEvent = (_err, page) => {
  console.log(page);
  // stop the concurrent crawl when 8 pages are found.
  if (website.size >= 8) {
    website.stop();
  }
};

await website.crawl(onPageEvent);
```
