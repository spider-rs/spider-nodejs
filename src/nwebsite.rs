use crate::NPage;

#[napi]
/// website main data from rust to node.
pub struct NWebsite {
  /// all of the website links.
  pub links: Vec<String>,
  /// the pages found.
  pub pages: Vec<NPage>,
}
