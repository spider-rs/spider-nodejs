#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

/// a simple page object dummy
#[derive(Default, Clone)]
#[napi(object)]
pub struct Page {
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
  pub pages: Vec<Page>,
}

#[napi]
/// crawl a website gathering all links to array
pub async fn crawl(url: String) -> NWebsite {
  let mut website = spider::website::Website::new(&url);
  let mut rx2 = website
    .subscribe(16)
    .expect("sync feature should be enabled");
  let (tx, mut rx) = spider::tokio::sync::mpsc::channel(333);

  spider::tokio::spawn(async move {
    while let Ok(res) = rx2.recv().await {
      let url = res.get_url();

      if let Err(_) = tx
        .send(Page {
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

  /// get all the pages of a website
  #[napi]
  pub fn get_pages(&self) -> Vec<Page> {
    let mut pages: Vec<Page> = Vec::new();

    match self.inner.get_pages() {
      Some(p) => {
        for page in p.iter() {
          pages.push(Page {
            url: page.get_url().into(),
            content: page.get_html(),
          });
        }
      }
      _ => (),
    }

    pages
  }
}
