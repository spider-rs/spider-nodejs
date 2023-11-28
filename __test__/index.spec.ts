import test from "ava";
import { crawl, Website, type NPage, Cron } from "../index.js";

const TEST_URL = "https://choosealicense.com";

test("crawl native", async (t) => {
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
