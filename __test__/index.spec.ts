import test from "ava";
import { crawl, Website, Page, type NPage, Cron, pageTitle } from "../index.js";

const TEST_URL = "https://choosealicense.com";

test("shortcut crawl native", async (t) => {
  const { links, pages } = await crawl(TEST_URL);

  t.assert(links.length > 1, "should be more than one link");
  t.assert(pages.length > 1, "should be more than one page");
});

test("new website native", async (t) => {
  const website = new Website(TEST_URL);
  await website.crawl();

  t.assert(website.getLinks().length > 1, "should be more than one link");
});

test("new website scrape native", async (t) => {
  const website = new Website(TEST_URL);
  await website.scrape();

  t.assert(website.getPages().length > 1, "should be more than one page");
});

test("new website native with custom config", async (t) => {
  const website = new Website(TEST_URL)
    .withHeaders({
      authorization: "somerandomjwt",
    })
    .build();

  await website.crawl();

  t.assert(website.getLinks().length > 1, "should be more than one page");
});

test("new website native budget one page", async (t) => {
  const website = new Website(TEST_URL)
    .withBudget({
      "*": 1,
    })
    .build();

  await website.crawl();

  t.assert(website.getLinks().length === 1, "should be one link");
});

test("new website native blacklist pages", async (t) => {
  const website = new Website(TEST_URL)
    .withBlacklistUrl(["/blog", new RegExp("/books").source, "/resume"])
    .build();

  await website.crawl();

  const links = website.getLinks();

  // should be valid unless new pages and routes are created.
  t.assert(
    links.length > 1 && !links.includes(`${TEST_URL}/blog`),
    "should be more than one page",
  );
});

test("new website native onPageEvent", async (t) => {
  const website = new Website(TEST_URL);

  const links: NPage[] = [];

  const onPageEvent = (err: Error | null, value: NPage) => {
    links.push(value);
  };

  // running in background can be done with a sleep timer for test.
  const backgroundStream = false;

  await website.crawl(onPageEvent, backgroundStream);

  // should be valid unless new pages and routes are created.
  t.assert(links.length > 1, "should be more than one page");
});

test("new website native with title selector", async (t) => {
  const website = new Website(TEST_URL);

  const links: { url: string; title: string }[] = [];

  const onPageEvent = async (_err: Error | null, page: NPage) => {
    const title = pageTitle(page);
    links.push({ title, url: page.url });
  };

  await website.crawl(onPageEvent);

  // should be valid unless new pages and routes are created.
  t.assert(links.length > 1, "should be more than one page");
});

// experimental - does not work on all platforms most likely due to time differences.
test.skip("new website native cron", async (t) => {
  const website = new Website(TEST_URL).withCron("1/5 * * * * *");
  // sleep function to test cron
  const sleep = (time: number, handle: Cron) => {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve(handle.stop());
      }, time);
    });
  };

  const links: NPage[] = [];

  const onPageEvent = (err: Error | null, value: NPage) => {
    links.push(value);
  };

  const handle = await website.runCron(onPageEvent);

  await sleep(4000, handle);

  // should be valid unless new pages and routes are created.
  t.assert(links.length > 1, "should be more than one page");
});

test("new website native with subscriptions", async (t) => {
  const website = new Website(TEST_URL);

  const links: NPage[] = [];

  const onPageEvent = (_err: Error | null, value: NPage) => {
    links.push(value);
  };

  const id = website.subscribe(onPageEvent);

  await website.crawl();

  website.unsubscribe(id);

  // should be valid unless new pages and routes are created.
  t.assert(links.length > 1, "should be more than one page");
});

test("new single page", async (t) => {
  const page = new Page(TEST_URL);
  await page.fetch();
  const links = await page.getLinks();

  // should be valid unless new pages and routes are created.
  t.assert(links.length > 1, "should be more than one link");
  t.assert(page.getHtml().length >= 100, "should be valid html");
  t.assert(page.getBytes().length >= 100, "should be valid bytes");
});

test.skip("new website native headless", async (t) => {
  const website = new Website(TEST_URL);
  await website.crawl(undefined, false, true);

  t.assert(website.getLinks().length > 1, "should be more than one link");
});

test("new website native raw content", async (t) => {
  const website = new Website(TEST_URL, true);

  const links: Buffer[] = [];

  const onPageEvent = (_err: Error | null, page: NPage) =>
    page.rawContent && links.push(page.rawContent);

  await website.crawl(onPageEvent);

  t.assert(links.length > 1, "should be more than one page");
});


test("new website data store and export", async (t) => {
  const { promises } = await import('node:fs');
  const readFile = promises.readFile;
  
  const website = new Website(TEST_URL, true);
  const outputFile = "./storage/test.jsonl";

  const onPageEvent = (_err: Error | null, page: NPage) => {
    website.pushData(page);
  };

  await website.crawl(onPageEvent);
  await website.exportJsonlData(outputFile);

  const data = await readFile(outputFile);

  t.assert(!!data, "should contain valid json file");
});

test("new website stop", async (t) => {  
  const website = new Website(TEST_URL);

  const onPageEvent = async (_err: Error | null, page: NPage) => {
    if (website.size >= 8) {
      await website.stop();
    }
  };

  await website.crawl(onPageEvent);

  t.assert(website.size < 15, "should only have crawled a couple pages concurrently");
});

test("new website stop background", async (t) => {
  const sleep = (time: number) => {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve(true);
      }, time);
    });
  };

  const website = new Website(TEST_URL);
  let count = 0;

  const onPageEvent = async (_err: Error | null, page: NPage) => {
    if (count) {
      await website.stop();
    }
    count++;
  };

  // lets wait for all other test since background shutsdown all crawls matching the url
  await sleep(2000);
  await website.crawl(onPageEvent, true);
  await sleep(2000);

  t.assert(count < 15, "should only have crawled a couple pages concurrently in the background");
});

