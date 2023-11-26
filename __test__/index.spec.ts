import test from 'ava'

import { collectAllLinks } from '../index.js'

test('collect links native', async (t) => {
  const { links } = await collectAllLinks("https://rsseau.fr");

  t.assert(links.length > 1, "should be more than one")
})

test('collect pages native', async (t) => {
  const { pages } = await collectAllLinks("https://rsseau.fr");

  t.assert(pages.length > 1, "should be more than one")
})
