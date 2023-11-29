#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod page;
pub mod website;

pub use page::Page;
pub use website::{crawl, NPage, NWebsite, Website};
