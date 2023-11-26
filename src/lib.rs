#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct Website {
  /// all of the website links.
  pub links: Vec<String>,
}

#[napi]
/// crawl a website gathering all links to array
pub async fn collect_all_links(n: String) -> Website {
  let mut website = spider::website::Website::new(&n);
  website.crawl().await;
  let links = website
    .get_links()
    .into_iter()
    .map(|x| x.as_ref().to_string())
    .collect::<Vec<String>>();

  Website { links }
}
