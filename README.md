# spider-nodejs

The [spider](https://github.com/spider-rs/spider) project ported to nodejs via napi.

## Getting Started

1. `npm i @spider-rs/spider-rs --save`

```ts
import { crawl } from '@spider-rs/spider-rs'

// gather all the links found in a website fast concurrently.
const { links, pages } = await crawl("https://rsseau.fr");
```

## Development

Install the napi cli `npm i @napi-rs/cli --global`.

1. `yarn build:test`

### TODO: Full Spider Port

Port the modules to be used via nodejs to adhere spider interface.

A full port would require FromNapi support on the following modules.

- compact_str
- case_insensitive_str
- small_vec