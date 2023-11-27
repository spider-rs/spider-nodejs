# spider-nodejs

The [spider](https://github.com/spider-rs/spider) project ported to nodejs via napi.

## Getting Started

1. `npm i @spider-rs/spider-rs --save`

```ts
import { Website } from '@spider-rs/spider-rs'

const website = new Website("https://rsseau.fr");
await website.crawl();
console.log(website.getLinks())
```

Collect the resource.

```ts
import { Website } from '@spider-rs/spider-rs'

const website = new Website("https://rsseau.fr");
await website.scrape();
console.log(website.getPages())
```

Use the crawl shortcut to get the page content and url.

```ts
import { crawl } from '@spider-rs/spider-rs'

const { links, pages } = new crawl("https://rsseau.fr");
console.log(pages)
```


## Development

Install the napi cli `npm i @napi-rs/cli --global`.

1. `yarn build:test`