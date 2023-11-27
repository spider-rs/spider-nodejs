import test from 'ava'

import { crawl, Website } from '../index.js'

test('crawl native', async (t) => {
  const { links, pages } = await crawl("https://rsseau.fr");

  t.assert(links.length > 1, "should be more than one link")
  t.assert(pages.length > 1, "should be more than one page")
})

test('new website native', async (t) => {
  const website = new Website("https://rsseau.fr");
  await website.crawl();
 
  t.assert(website.getLinks().length > 1, "should be more than one link")
})

test('new website scrape native', async (t) => {
  const website = new Website("https://rsseau.fr");
  await website.scrape();

  t.assert(website.getPages().length > 1, "should be more than one page")
})
