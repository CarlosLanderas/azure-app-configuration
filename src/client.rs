use crate::request::Request;
use crate::Exception;
use http::{Method, Uri, Response};
use url::Url;

pub struct AzureAppConfigClient {
    uri_endpoint: Uri,
    access_key: String,
    secret: Vec<u8>,
}

impl AzureAppConfigClient {
    pub fn new<S: Into<String>>(
        uri_endpoint: S,
        access_key: S,
        secret: Vec<u8>,
    ) -> AzureAppConfigClient {
        AzureAppConfigClient {
            uri_endpoint: uri_endpoint.into().parse::<Uri>().unwrap(),
            access_key: access_key.into(),
            secret: secret.into(),
        }
    }

    pub async fn get_keys(&self) -> Result<String, Exception> {
        let body = vec![];
        let url = "https://lande-app-configuration.azconfig.io/kv?fields=*"
            .parse::<Url>()
            .unwrap();
        let req = crate::request::Request::create_signed_request(
            self.access_key.clone(),
            self.secret.clone(),
            url,
            body,
            Method::GET,
        )
        .await?;

        let mut result  = req.await?;
        println!("Status code: {}", result.status());
        let response_str: String = result.body_string().await?;


        println!("{}", response_str);

        Ok(String::from("Carlo"))
    }
}
