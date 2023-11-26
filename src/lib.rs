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
pub struct Website {
  /// all of the website links.
  pub links: Vec<String>,
  /// the pages found
  pub pages: Vec<Page>,
}

#[napi]
/// crawl a website gathering all links to array
pub async fn crawl(n: String) -> Website {
  let mut website = spider::website::Website::new(&n);
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

  Website { links, pages }
}
