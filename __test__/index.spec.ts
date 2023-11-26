import test from 'ava'

import { crawl } from '../index.js'

test('crawl native', async (t) => {
  const { links, pages } = await crawl("https://rsseau.fr");

  t.assert(links.length > 1, "should be more than one link")
  t.assert(pages.length > 1, "should be more than one page")
})