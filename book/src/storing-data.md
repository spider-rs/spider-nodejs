# Storing Data

Storing data can be done to collect the raw content for a website.

This allows you to upload and download the content without UTF-8 conversion. The property only appears when
setting the second param of the `Website` class constructor to true.

```ts
const rawContent = true

const links: Buffer[] = []

const onPageEvent = (_err: Error | null, page: NPage) => {
  if (page.rawContent) {
    // we can download or store the content now to disk.
    links.push(page.rawContent)
  }
}

await website.crawl(onPageEvent)

const website = new Website('https://choosealicense.com', rawContent)
```
