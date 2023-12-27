use compact_str::CompactString;
use napi;

/// a simple page object
#[napi]
#[derive(Default)]
pub struct Page {
  /// the page object from spider
  inner: Option<spider::page::Page>,
  /// selectors
  selectors: Option<(
    CompactString,
    spider::smallvec::SmallVec<[CompactString; 2]>,
  )>,
  /// the url for the page
  pub url: String,
  pub subdomains: Option<bool>,
  pub tld: Option<bool>,
  pub status_code: u16,
}

#[napi]
impl Page {
  #[napi(constructor)]
  /// a new page
  pub fn new(url: String, subdomains: Option<bool>, tld: Option<bool>) -> Self {
    Page {
      url,
      subdomains,
      tld,
      ..Default::default()
    }
  }

  #[napi]
  /// get the page content
  pub async unsafe fn fetch(&mut self) -> &Self {
    use spider::{
      lazy_static::lazy_static, reqwest::Client, reqwest_middleware::ClientWithMiddleware,
      ClientBuilder,
    };
    lazy_static! {
      /// top level single page client to re-use.
      pub static ref PAGE_CLIENT: ClientWithMiddleware = {
        let reqwest_client = Client::builder().build().unwrap_or_default();
        let client = ClientBuilder::new(reqwest_client).build();

        client
      };
    }
    let page = spider::page::Page::new_page(&self.url, &PAGE_CLIENT).await;
    self.status_code = page.status_code.into();
    self.inner = Some(page);
    self.selectors = spider::page::get_page_selectors(
      &self.url,
      self.subdomains.unwrap_or_default(),
      self.tld.unwrap_or_default(),
    );
    self
  }

  #[napi]
  /// all links on the page
  pub async fn get_links(&self) -> Vec<String> {
    match &self.selectors {
      Some(selectors) => match &self.inner {
        Some(inner) => {
          let links = inner.links(&selectors).await;
          links
            .into_iter()
            .map(|i| i.as_ref().to_string())
            .collect::<Vec<String>>()
        }
        _ => Default::default(),
      },
      _ => Default::default(),
    }
  }

  #[napi]
  /// get the html for the page
  pub fn get_html(&self) -> String {
    match &self.inner {
      Some(inner) => inner.get_html(),
      _ => Default::default(),
    }
  }

  #[napi]
  /// get the bytes for the page
  pub fn get_bytes(&self) -> &[u8] {
    match &self.inner {
      Some(inner) => inner.get_html_bytes_u8(),
      _ => Default::default(),
    }
  }
}
