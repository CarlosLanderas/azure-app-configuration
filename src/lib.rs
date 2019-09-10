#[macro_use]
extern crate serde_derive;

type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;

pub mod client;
mod endpoints;
pub mod model;
mod request_sign;
