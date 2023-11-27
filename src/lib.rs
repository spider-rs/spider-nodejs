#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::Object;
use spider::lazy_static::lazy_static;

/// a simple page object
#[derive(Default, Clone)]
#[napi(object)]
pub struct NPage {
  /// the url found
  pub url: String,
  /// the content of the page found
  pub content: String,
}

#[napi]
/// website main data from rust to node
pub struct NWebsite {
  /// all of the website links.
  pub links: Vec<String>,
  /// the pages found
  pub pages: Vec<NPage>,
}

#[napi]
/// crawl a website gathering all links to array
pub async fn crawl(url: String) -> NWebsite {
  let mut website = spider::website::Website::new(&url);
  let mut rx2 = website
    .subscribe(16)
    .expect("sync feature should be enabled");

  lazy_static! {
    pub static ref BUFFER: usize = (num_cpus::get() * 20).max(88);
  }

  let (tx, mut rx) = spider::tokio::sync::mpsc::channel(*BUFFER);

  spider::tokio::spawn(async move {
    while let Ok(res) = rx2.recv().await {
      let url = res.get_url();

      if let Err(_) = tx
        .send(NPage {
          url: url.into(),
          content: res.get_html(),
        })
        .await
      {
        println!("receiver dropped");
        return;
      }
    }
  });

  spider::tokio::spawn(async move {
    website.crawl().await;
  });

  let mut pages = Vec::new();

  while let Some(i) = rx.recv().await {
    pages.push(i)
  }

  let links = pages.iter().map(|x| x.url.clone()).collect::<Vec<String>>();

  NWebsite { links, pages }
}

#[napi]
/// a website holding the inner spider::website::Website from Rust fit for nodejs
pub struct Website {
  /// the website from spider
  inner: spider::website::Website,
}

#[napi]
impl Website {
  #[napi(constructor)]
  pub fn new(url: String) -> Self {
    Website {
      inner: spider::website::Website::new(&url),
    }
  }
  #[napi]
  /// crawl a website
  pub async unsafe fn crawl(&mut self) {
    self.inner.crawl().await;
  }

  #[napi]
  /// scrape a website
  pub async unsafe fn scrape(&mut self) {
    self.inner.scrape().await;
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

    match self.inner.get_pages() {
      Some(p) => {
        for page in p.iter() {
          pages.push(NPage {
            url: page.get_url().into(),
            content: page.get_html(),
          });
        }
      }
      _ => (),
    }

    pages
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

  #[napi]
  pub fn with_budget(&mut self, budget: Option<std::collections::HashMap<String, u32>>) -> &Self {
    use spider::hashbrown::hash_map::HashMap;

    match budget {
      Some(d) => {
        let v = d
          .iter()
          .map(|e| e.0.to_owned() + "," + &e.1.to_string())
          .collect::<Vec<_>>();
        let v = v.join("");
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
