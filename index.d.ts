/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

/** a simple page object */
export interface NPage {
  /** The url found. */
  url: string
  /** The content of the page found. */
  content: string
  /** The HTTP status code. */
  statusCode: number
  /** The Raw content if the resource needs to be sent as binary. */
  rawContent?: Buffer
  /** The HTTP headers. */
  headers?: Record<string, string>
  /** The links found on the page. Requires the website.builder method website.with_subscription_return_page_links to be set to true. */
  links?: Array<string>
}
/** get the page title. */
export declare function pageTitle(page: NPage): string
/** crawl a website using HTTP gathering all links and html. */
export declare function crawl(url: string, rawContent?: boolean | undefined | null): Promise<NWebsite>
export interface PageEvent {
  page: NPage
}
/** website main data from rust to node. */
export class NWebsite {
  /** all of the website links. */
  links: Array<string>
  /** the pages found. */
  pages: Array<NPage>
}
/** a simple page object */
export class Page {
  /** The url for the page. */
  url: string
  /** The website crawling subdomain pages? */
  subdomains?: boolean
  /** The website crawling TLD pages? */
  tld?: boolean
  /** The HTTP status code. */
  statusCode: number
  /** a new page */
  constructor(url: string, subdomains?: boolean | undefined | null, tld?: boolean | undefined | null)
  /** get the page content */
  fetch(): Promise<this>
  /** all links on the page */
  getLinks(): Promise<Array<string>>
  /** get the html for the page */
  getHtml(): string
  /** get the bytes for the page */
  getBytes(): Uint8Array
}
/** a website holding the inner spider::website::Website from Rust fit for nodejs. */
export class Website {
  /** a new website. */
  constructor(url: string, rawContent?: boolean | undefined | null)
  /** Get the crawl status. */
  get status(): string
  /** Store data to heap memory. The data must be an object. Use `website.export_jsonl_data` to store to disk. When using this method test occordingly since only certain primitives are supported. */
  pushData(obj: unknown): void
  /** Clear the collected data from heap memory. This only handles the data from `website.pushData`. */
  clearData(): void
  /** read the data from the heap memory. */
  readData(): any
  /** store data to memory for disk storing. This will create the path if not exist and defaults to ./storage. */
  exportJsonlData(exportPath?: string | undefined | null): Promise<void>
  /** subscribe and add an event listener. */
  subscribe(onPageEvent: (err: Error | null, arg: NPage) => any): number
  /** remove a subscription listener. */
  unsubscribe(id?: number | undefined | null): boolean
  /** stop a crawl */
  stop(id?: number | undefined | null): Promise<boolean>
  /** crawl a website */
  crawl(onPageEvent?: (err: Error | null, arg: NPage) => any | undefined | null, background?: boolean | undefined | null, headless?: boolean | undefined | null): Promise<void>
  /** Start to crawl website with async concurrency smart. Use HTTP first and JavaScript Rendering as needed. */
  crawlSmart(onPageEvent?: (err: Error | null, arg: NPage) => any | undefined | null, background?: boolean | undefined | null): Promise<void>
  /** scrape a website */
  scrape(onPageEvent?: (err: Error | null, arg: NPage) => any | undefined | null, background?: boolean | undefined | null, headless?: boolean | undefined | null): Promise<void>
  /** run a cron job */
  runCron(onPageEvent?: (err: Error | null, arg: NPage) => any | undefined | null): Promise<Cron>
  /** get all the links of a website */
  getLinks(): Array<string>
  /** get the size of the website in amount of pages crawled. If you ran the page in the background, this value will not update. */
  get size(): number
  /** get all the pages of a website - requires calling website.scrape */
  getPages(): Array<NPage>
  /** drain all links from storing */
  drainLinks(): Array<string>
  /** clear all links and page data */
  clear(): void
  /** Set HTTP headers for request using [reqwest::header::HeaderMap](https://docs.rs/reqwest/latest/reqwest/header/struct.HeaderMap.html). */
  withHeaders(headers?: object | undefined | null): this
  /** Add user agent to request. */
  withUserAgent(userAgent?: string | undefined | null): this
  /** Respect robots.txt file. */
  withRespectRobotsTxt(respectRobotsTxt: boolean): this
  /** Determine whether to collect all the resources found on pages. */
  withFullResources(fullResources: boolean): this
  /** Use network interception for the request to only allow content that matches the host. If the content is from a 3rd party it needs to be part of our include list. */
  withChromeIntercept(chromeIntercept: boolean, blockImages: boolean): this
  /** Set the connection url for the chrome instance. This method does nothing if the `chrome` is not enabled. */
  withChromeConnection(chromeConnection: string): this
  /** Preserve the HOST header. */
  withPreserveHostHeader(preserveHost: boolean): this
  /** Include subdomains detection. */
  withSubdomains(subdomains: boolean): this
  /** Include tld detection. */
  withTld(tld: boolean): this
  /** Only use HTTP/2. */
  withHttp2PriorKnowledge(http2PriorKnowledge: boolean): this
  /** Max time to wait for request duration to milliseconds. */
  withRequestTimeout(requestTimeout?: number | undefined | null): this
  /** add external domains */
  withExternalDomains(externalDomains?: Array<string> | undefined | null): this
  /** Use stealth mode for the request. This does nothing without chrome. */
  withStealth(stealthMode?: boolean | undefined | null): this
  /** Set the crawling budget */
  withBudget(budget?: Record<string, number> | undefined | null): this
  /** Set the max redirects allowed for request. */
  withRedirectLimit(redirectLimit: number): this
  /** Set the redirect policy to use, either Strict or Loose by default. */
  withRedirectPolicy(strict: boolean): this
  /** Regex blacklist urls from the crawl */
  withBlacklistUrl(blacklistUrl?: Array<string> | undefined | null): this
  /** Regex whitelist urls from the crawl */
  withWhitelistUrl(whitelistUrl?: Array<string> | undefined | null): this
  /** Wait for a delay. Should only be used for testing. This method does nothing if the `chrome` feature is not enabled. */
  withWaitForDelay(seconds?: number | undefined | null, nanos?: number | undefined | null): this
  /** Wait for a CSS query selector. This method does nothing if the `chrome` feature is not enabled. */
  withWaitForSelector(selector?: string | undefined | null, seconds?: number | undefined | null, nanos?: number | undefined | null): this
  /** Wait for idle network request. This method does nothing if the `chrome` feature is not enabled. */
  withWaitForIdleNetwork(seconds?: number | undefined | null, nanos?: number | undefined | null): this
  /** Setup cron jobs to run */
  withCron(cronStr: string, cronType?: string | undefined | null): this
  /** Use OpenAI to generate dynamic javascript snippets. Make sure to set the `OPENAI_API_KEY` env variable. */
  withOpenai(openaiConfigs?: object | undefined | null): this
  /** Take screenshots of web pages using chrome. */
  withScreenshot(screenshotConfigs?: {
  /** The screenshot params. */
  params: {
  /** Chrome DevTools Protocol screenshot options. */
  cdp_params: {
  /** Image compression format (defaults to png). */
  format: 'jpeg' | 'png' | 'webp'
  /** Compression quality from range [0..100] (jpeg only). */
  quality: number
  /** Capture the screenshot of a given region only. */
  clip: {
  x: number
  y: number
  height: number
  width: number
  scale: number
  }
  /** Capture the screenshot from the surface, rather than the view. Defaults to true.*/
  from_surface: boolean
  /** Capture the screenshot beyond the viewport. Defaults to false. */
  capture_beyond_viewport: boolean
  }
  /** Take full page screenshot */
  full_page: boolean
  /** Make the background transparent (png only). */
  omit_background: boolean
  }
  /** Return the bytes of the screenshot on the Page. */
  bytes: boolean
  /** Store the screenshot to disk. This can be used with output_dir. If disabled will not store the file to the output directory. */
  save: boolean
  /** The output directory to store the file. Parent folders may be created inside the directory. */
  output_dir: string | null
  }): this
  /** Delay between request as ms. */
  withDelay(delay: number): this
  /** Set a crawl depth limit. If the value is 0 there is no limit. */
  withDepth(depth: number): this
  /** Return the links found on the page in the channel subscriptions. This method does nothing if the `decentralized` is enabled. */
  withReturnPageLinks(returnPageLinks: boolean): this
  /** Cache the page following HTTP rules. */
  withCaching(cache: boolean): this
  /** Set the sitemap url. */
  withSitemap(sitemap?: string | undefined | null): this
  /** Use proxies for request. */
  withProxies(proxies?: Array<string> | undefined | null): this
  /** build the inner website - not required for all builder_steps */
  build(): this
}
/** a runner for handling crons */
export class Cron {
  /** stop the cron instance */
  stop(): Promise<void>
}
