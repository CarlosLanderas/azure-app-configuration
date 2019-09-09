use crate::request::Request;
use http::Uri;

pub struct AzureAppConfigClient<'a> {
    uri_endpoint: Uri,
    access_key: &'a str,
    secret: &'a str,
}

impl<'a> AzureAppConfigClient<'a> {
    pub fn new<S: AsRef<str>>(
        uri_endpoint: S,
        access_key: &'a S,
        secret: &'a S,
    ) -> AzureAppConfigClient<'a> {
        AzureAppConfigClient {
            uri_endpoint: uri_endpoint.as_ref().parse::<Uri>().unwrap(),
            access_key: access_key.as_ref(),
            secret: secret.as_ref(),
        }
    }

    pub fn get_keys() {
        Request {};
    }
}
