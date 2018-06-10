extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod scrape;

pub mod error;
pub mod image;
pub mod profile;
