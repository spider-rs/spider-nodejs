use napi::bindgen_prelude::Buffer;
use spider::{
  lazy_static::lazy_static,
  packages::scraper::{Html, Selector},
  reqwest::header::HeaderMap,
};
use std::collections::HashMap;

/// a simple page object
#[derive(Default, Clone)]
#[napi(object)]
pub struct NPage {
  /// The url found.
  pub url: String,
  /// The content of the page found.
  pub content: String,
  /// The HTTP status code.
  pub status_code: u16,
  /// The Raw content if the resource needs to be sent as binary.
  pub raw_content: Option<Buffer>,
  /// The HTTP headers.
  pub headers: Option<HashMap<String, String>>,
  /// The links found on the page. Requires the website.builder method website.with_subscription_return_page_links to be set to true.
  pub links: Option<Vec<String>>,
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
      headers: match res.headers {
        Some(ref headers) => Some(header_map_to_hash_map(headers)),
        _ => None,
      },
      links: match res.page_links {
        Some(ref links) => Some(
          links
            .iter()
            .map(|link| link.as_ref().to_string())
            .collect::<Vec<String>>(),
        ),
        _ => None,
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

/// convert a headermap to hashmap
pub fn header_map_to_hash_map(header_map: &HeaderMap) -> HashMap<String, String> {
  let mut hash_map = HashMap::new();

  for (key, value) in header_map.iter() {
    let key = key.as_str().to_string();

    if let Ok(value_str) = value.to_str() {
      hash_map.insert(key, value_str.to_string());
    }
  }

  hash_map
}
