use compact_str::CompactString;
use indexmap::IndexMap;
use napi::{
  bindgen_prelude::{Buffer, Object},
  tokio::task::JoinHandle,
};
use spider::{
  lazy_static::lazy_static,
  packages::scraper::{Html, Selector},
};
use std::time::Duration;

lazy_static! {
  pub static ref BUFFER: usize = (num_cpus::get() * 20).max(88);
}

/// a simple page object
#[derive(Default, Clone)]
#[napi(object)]
pub struct NPage {
  /// the url found.
  pub url: String,
  /// the content of the page found.
  pub content: String,
  /// the HTTP status code.
  pub status_code: u16,
  /// the raw content
  pub raw_content: Option<Buffer>,
}

#[napi]
/// get the page title.
pub fn page_title(page: NPage) -> String {
  page.title()
}

#[napi]
impl NPage {
  /// establish a new page
  pub fn new(res: &spider::page::Page, raw: bool) -> NPage {
    NPage {
      url: res.get_url().into(),
      status_code: res.status_code.as_u16(),
      content: if raw {
        Default::default()
      } else {
        res.get_html()
      },
      raw_content: if raw {
        Some(res.get_html_bytes_u8().into())
      } else {
        None
      },
    }
  }

  #[napi]
  /// the html page title.
  pub fn title(&self) -> String {
    lazy_static! {
      static ref TITLE_SELECTOR: Selector = Selector::parse("title").unwrap();
    }
    let fragment: Html = Html::parse_document(&self.content);
    match fragment.select(&TITLE_SELECTOR).next() {
      Some(title) => title.inner_html(),
      _ => Default::default(),
    }
  }
}

#[napi]
/// website main data from rust to node.
pub struct NWebsite {
  /// all of the website links.
  pub links: Vec<String>,
  /// the pages found.
  pub pages: Vec<NPage>,
}

#[napi]
/// crawl a website using HTTP gathering all links and html.
pub async fn crawl(url: String, raw_content: Option<bool>) -> NWebsite {
  let mut website = spider::website::Website::new(&url);
  let mut rx2 = website
    .subscribe(*BUFFER / 2)
    .expect("sync feature should be enabled");
  let (tx, mut rx) = spider::tokio::sync::mpsc::channel(*BUFFER);
  let raw_content = raw_content.unwrap_or_default();

  spider::tokio::spawn(async move {
    while let Ok(res) = rx2.recv().await {
      if let Err(_) = tx.send(NPage::new(&res, raw_content)).await {
        println!("receiver dropped");
        return;
      }
    }
  });

  spider::tokio::spawn(async move {
    website.crawl_raw().await;
  });

  let mut pages = Vec::new();

  while let Some(i) = rx.recv().await {
    pages.push(i)
  }

  let links = pages.iter().map(|x| x.url.clone()).collect::<Vec<String>>();

  NWebsite { links, pages }
}

#[napi]
/// a website holding the inner spider::website::Website from Rust fit for nodejs.
pub struct Website {
  /// the website from spider.
  inner: spider::website::Website,
  /// spawn subscription handles.
  subscription_handles: IndexMap<u32, JoinHandle<()>>,
  /// do not convert content to UT8.
  raw_content: bool,
}

#[napi]
impl Website {
  #[napi(constructor)]
  /// a new website.
  pub fn new(url: String, raw_content: Option<bool>) -> Self {
    Website {
      inner: spider::website::Website::new(&url),
      subscription_handles: IndexMap::new(),
      raw_content: raw_content.unwrap_or_default(),
    }
  }

  /// Get the crawl status.
  #[napi(getter)]
  pub fn status(&self) -> String {
    use std::string::ToString;
    self.inner.get_status().to_string()
  }

  #[napi]
  /// subscribe and add an event listener.
  pub fn subscribe(
    &mut self,
    on_page_event: napi::threadsafe_function::ThreadsafeFunction<NPage>,
  ) -> u32 {
    let mut rx2 = self
      .inner
      .subscribe(*BUFFER / 2)
      .expect("sync feature should be enabled");
    let raw_content = self.raw_content;

    let handle = spider::tokio::spawn(async move {
      while let Ok(res) = rx2.recv().await {
        on_page_event.call(
          Ok(NPage::new(&res, raw_content)),
          napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
        );
      }
    });

    // always return the highest value as the next id.
    let id = match self.subscription_handles.last() {
      Some(handle) => handle.0 + 1,
      _ => 0,
    };

    self.subscription_handles.insert(id, handle);

    id
  }

  #[napi]
  /// remove a subscription listener.
  pub fn unsubscribe(&mut self, id: Option<u32>) -> bool {
    match id {
      Some(id) => {
        let handle = self.subscription_handles.get(&id);

        match handle {
          Some(h) => {
            h.abort();
            self.subscription_handles.remove_entry(&id);
            true
          }
          _ => false,
        }
      }
      // we may want to get all subs and remove them
      _ => {
        let keys = self.subscription_handles.len();
        for k in self.subscription_handles.drain(..) {
          k.1.abort();
        }
        keys > 0
      }
    }
  }

  #[napi]
  /// crawl a website
  pub async unsafe fn crawl(
    &mut self,
    on_page_event: Option<napi::threadsafe_function::ThreadsafeFunction<NPage>>,
    // run the page in the background
    background: Option<bool>,
    // headless chrome rendering
    headless: Option<bool>,
  ) {
    // only run in background if on_page_event is handled for streaming.
    let background = background.is_some() && background.unwrap_or_default();
    let headless = headless.is_some() && headless.unwrap_or_default();
    let raw_content = self.raw_content;

    match on_page_event {
      Some(callback) => {
        if background {
          let mut website = self.inner.clone();

          let mut rx2 = website
            .subscribe(*BUFFER / 2)
            .expect("sync feature should be enabled");

          spider::tokio::spawn(async move {
            while let Ok(res) = rx2.recv().await {
              callback.call(
                Ok(NPage::new(&res, raw_content)),
                napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
              );
            }
          });

          spider::tokio::spawn(async move {
            if headless {
              website.crawl().await;
            } else {
              website.crawl_raw().await;
            }
          });
        } else {
          let mut rx2 = self
            .inner
            .subscribe(*BUFFER / 2)
            .expect("sync feature should be enabled");

          spider::tokio::spawn(async move {
            while let Ok(res) = rx2.recv().await {
              callback.call(
                Ok(NPage::new(&res, raw_content)),
                napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
              );
            }
          });

          if headless {
            self.inner.crawl().await;
          } else {
            self.inner.crawl_raw().await;
          }
        }
      }
      _ => {
        if headless {
          self.inner.crawl().await;
        } else {
          self.inner.crawl_raw().await;
        }
      }
    }
  }

  #[napi]
  /// scrape a website
  pub async unsafe fn scrape(
    &mut self,
    on_page_event: Option<napi::threadsafe_function::ThreadsafeFunction<NPage>>,
    background: Option<bool>,
    headless: Option<bool>,
  ) {
    let headless = headless.is_some() && headless.unwrap_or_default();
    let raw_content = self.raw_content;

    match on_page_event {
      Some(callback) => {
        if background.unwrap_or_default() {
          let mut website = self.inner.clone();

          let mut rx2 = website
            .subscribe(*BUFFER / 2)
            .expect("sync feature should be enabled");

          spider::tokio::spawn(async move {
            while let Ok(res) = rx2.recv().await {
              callback.call(
                Ok(NPage::new(&res, raw_content)),
                napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
              );
            }
          });

          spider::tokio::spawn(async move {
            if headless {
              website.scrape().await;
            } else {
              website.scrape_raw().await;
            }
          });
        } else {
          let mut rx2 = self
            .inner
            .subscribe(*BUFFER / 2)
            .expect("sync feature should be enabled");

          spider::tokio::spawn(async move {
            while let Ok(res) = rx2.recv().await {
              callback.call(
                Ok(NPage::new(&res, raw_content)),
                napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
              );
            }
          });

          if headless {
            self.inner.scrape().await;
          } else {
            self.inner.scrape_raw().await;
          }
        }
      }
      _ => {
        if headless {
          self.inner.scrape().await;
        } else {
          self.inner.scrape_raw().await;
        }
      }
    }
  }

  /// run the cron
  #[napi]
  pub async unsafe fn run_cron(
    &mut self,
    on_page_event: Option<napi::threadsafe_function::ThreadsafeFunction<NPage>>,
  ) -> Cron {
    let cron_handle = match on_page_event {
      Some(callback) => {
        let mut rx2 = self
          .inner
          .subscribe(*BUFFER / 2)
          .expect("sync feature should be enabled");
        let raw_content = self.raw_content;

        let handler = spider::tokio::spawn(async move {
          while let Ok(res) = rx2.recv().await {
            callback.call(
              Ok(NPage::new(&res, raw_content)),
              napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
            );
          }
        });

        Some(handler)
      }
      _ => None,
    };

    let inner = self.inner.run_cron().await;

    Cron { inner, cron_handle }
  }

  #[napi]
  /// get all the links of a website
  pub fn get_links(&self) -> Vec<String> {
    let links = self
      .inner
      .get_links()
      .iter()
      .map(|x| x.as_ref().to_string())
      .collect::<Vec<String>>();
    links
  }

  /// get all the pages of a website - requires calling website.scrape
  #[napi]
  pub fn get_pages(&self) -> Vec<NPage> {
    let mut pages: Vec<NPage> = Vec::new();
    let raw_content = self.raw_content;

    match self.inner.get_pages() {
      Some(p) => {
        for page in p.iter() {
          pages.push(NPage::new(page, raw_content));
        }
      }
      _ => (),
    }

    pages
  }

  #[napi]
  /// drain all links from storing
  pub fn drain_links(&mut self) -> Vec<String> {
    let links = self
      .inner
      .drain_links()
      .map(|x| x.as_ref().to_string())
      .collect::<Vec<String>>();

    links
  }

  #[napi]
  /// clear all links and page data
  pub fn clear(&mut self) {
    self.inner.clear();
  }

  #[napi]
  /// Set HTTP headers for request using [reqwest::header::HeaderMap](https://docs.rs/reqwest/latest/reqwest/header/struct.HeaderMap.html).
  pub fn with_headers(&mut self, headers: Option<Object>) -> &Self {
    use std::str::FromStr;

    match headers {
      Some(obj) => {
        let mut h = spider::reqwest::header::HeaderMap::new();
        let keys = Object::keys(&obj).unwrap_or_default();

        for key in keys.into_iter() {
          let header_key = spider::reqwest::header::HeaderName::from_str(&key);

          match header_key {
            Ok(hn) => {
              let header_value = obj
                .get::<String, String>(key)
                .unwrap_or_default()
                .unwrap_or_default();

              match spider::reqwest::header::HeaderValue::from_str(&header_value) {
                Ok(hk) => {
                  h.append(hn, hk);
                }
                _ => (),
              }
            }
            _ => (),
          }
        }
        self.inner.with_headers(Some(h));
      }
      _ => {
        self.inner.with_headers(None);
      }
    };

    self
  }

  /// Add user agent to request.
  #[napi]
  pub fn with_user_agent(&mut self, user_agent: Option<&str>) -> &Self {
    self.inner.configuration.with_user_agent(user_agent);
    self
  }

  /// Respect robots.txt file.
  #[napi]
  pub fn with_respect_robots_txt(&mut self, respect_robots_txt: bool) -> &Self {
    self
      .inner
      .configuration
      .with_respect_robots_txt(respect_robots_txt);
    self
  }

  /// Include subdomains detection.
  #[napi]
  pub fn with_subdomains(&mut self, subdomains: bool) -> &Self {
    self.inner.configuration.with_subdomains(subdomains);
    self
  }

  /// Include tld detection.
  #[napi]
  pub fn with_tld(&mut self, tld: bool) -> &Self {
    self.inner.configuration.with_tld(tld);
    self
  }

  /// Only use HTTP/2.
  #[napi]
  pub fn with_http2_prior_knowledge(&mut self, http2_prior_knowledge: bool) -> &Self {
    self
      .inner
      .configuration
      .with_http2_prior_knowledge(http2_prior_knowledge);
    self
  }

  /// Max time to wait for request duration to milliseconds.
  #[napi]
  pub fn with_request_timeout(&mut self, request_timeout: Option<u32>) -> &Self {
    self
      .inner
      .configuration
      .with_request_timeout(match request_timeout {
        Some(d) => Some(Duration::from_millis(d.into())),
        _ => None,
      });
    self
  }

  /// add external domains
  #[napi]
  pub fn with_external_domains(&mut self, external_domains: Option<Vec<String>>) -> &Self {
    self.inner.with_external_domains(match external_domains {
      Some(ext) => Some(ext.into_iter()),
      _ => None,
    });
    self
  }

  #[napi]
  /// Set the crawling budget
  pub fn with_budget(&mut self, budget: Option<std::collections::HashMap<String, u32>>) -> &Self {
    use spider::hashbrown::hash_map::HashMap;

    match budget {
      Some(d) => {
        let v = d
          .iter()
          .map(|e| e.0.to_owned() + "," + &e.1.to_string())
          .collect::<String>();
        let v = v
          .split(",")
          .collect::<Vec<_>>()
          .chunks(2)
          .map(|x| (x[0], x[1].parse::<u32>().unwrap_or_default()))
          .collect::<HashMap<&str, u32>>();

        self.inner.with_budget(Some(v));
      }
      _ => (),
    }

    self
  }

  #[napi]
  /// Regex black list urls from the crawl
  pub fn with_blacklist_url(&mut self, blacklist_url: Option<Vec<String>>) -> &Self {
    self
      .inner
      .configuration
      .with_blacklist_url(match blacklist_url {
        Some(v) => {
          let mut blacklist: Vec<CompactString> = Vec::new();
          for item in v {
            blacklist.push(CompactString::new(item));
          }
          Some(blacklist)
        }
        _ => None,
      });

    self
  }

  /// Setup cron jobs to run
  #[napi]
  pub fn with_cron(&mut self, cron_str: String, cron_type: Option<String>) -> &Self {
    self.inner.with_cron(
      cron_str.as_str(),
      if cron_type.unwrap_or_default() == "scrape" {
        spider::website::CronType::Scrape
      } else {
        spider::website::CronType::Crawl
      },
    );
    self
  }

  /// Delay between request as ms.
  #[napi]
  pub fn with_delay(&mut self, delay: u32) -> &Self {
    self.inner.configuration.with_delay(delay.into());
    self
  }

  /// Use proxies for request.
  #[napi]
  pub fn with_proxies(&mut self, proxies: Option<Vec<String>>) -> &Self {
    self.inner.configuration.with_proxies(proxies);
    self
  }

  #[napi]
  /// build the inner website - not required for all builder_steps
  pub fn build(&mut self) -> &Self {
    match self.inner.build() {
      Ok(w) => self.inner = w,
      _ => (),
    }
    self
  }
}

/// a runner for handling crons
#[napi]
pub struct Cron {
  /// the runner task
  inner: spider::async_job::Runner,
  /// inner cron handle
  cron_handle: Option<JoinHandle<()>>,
}

#[napi]
impl Cron {
  /// stop the cron instance
  #[napi]
  pub async unsafe fn stop(&mut self) {
    self.inner.stop().await;
    match &self.cron_handle {
      Some(h) => h.abort(),
      _ => (),
    }
  }
}
