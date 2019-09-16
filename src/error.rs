use serde::export::fmt::Display;
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct HttpError {
    status: usize,
    url: String,
}

impl HttpError {
    pub fn new<S: Into<String>>(err: usize, url: S) -> Self {
        HttpError {
            status: err,
            url: url.into(),
        }
    }
}

impl Error for HttpError {}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Http request error, code: {}, url: {}",
            self.status, self.url
        )
    }
}
