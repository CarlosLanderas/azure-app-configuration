type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;

pub mod client;
mod endpoints;
mod request;
