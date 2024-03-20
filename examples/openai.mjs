// npm i @spider-rs/spider-rs
// node openai.mjs
import { Website } from "../index.js";

const url = process.argv[2] || "https://google.com";
const headless = true;

const website = new Website(url)
  .withBudget({ "*": 1 })
  .withScreenshot({
    params: {
        cdp_params: null,
        full_page: true,
        omit_background: false
    },
    bytes: false,
    save: true,
    output_dir: null
  })
  .withOpenai({
    model: "gpt-4-1106-preview",
    prompt: "Search for movies",
    max_tokens: 100,
  });

const onPageEvent = (_err, value) => {
  console.log(`Found: ${value.url}\nHTML: ${value.content}`);
};

const startTime = performance.now();

await website.crawl(onPageEvent, false, headless);

const duration = performance.now() - startTime;

console.log(
  "Finished",
  url,
  "pages found " + website.getLinks().length,
  "elasped duration " + duration + "ms",
);
