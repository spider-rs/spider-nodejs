use napi::bindgen_prelude::Buffer;
use spider::{
  lazy_static::lazy_static,
  packages::scraper::{Html, Selector},
};

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
