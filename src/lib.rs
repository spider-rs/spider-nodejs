#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use spider::website::Website;

#[napi]
/// crawl a website gathering all links to array
pub async fn collect_all_links(n: String) -> Vec<String> {
  let mut website = Website::new(&n);
  website.crawl().await;

  website
    .get_links()
    .into_iter()
    .map(|x| x.as_ref().to_string())
    .collect::<Vec<String>>()
}
