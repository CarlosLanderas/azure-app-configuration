use crate::endpoints::{EndpointUrl, Endpoints};
use crate::model::{KeyValue, KeyValues};
use crate::request_sign::create_signed_request;
use crate::requests::KeyValueRequest;
use crate::Exception;
use http::{Method, Response, Uri};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use surf::middleware::HttpClient;
use url::Url;

pub struct AzureAppConfigClient {
    access_key: String,
    secret: Vec<u8>,
    endpoints: Endpoints,
}

impl AzureAppConfigClient {
    pub fn new<S: Into<String>>(
        uri_endpoint: S,
        access_key: S,
        secret: Vec<u8>,
    ) -> AzureAppConfigClient {
        AzureAppConfigClient {
            access_key: access_key.into(),
            secret: secret.into(),
            endpoints: Endpoints::new(uri_endpoint.into()),
        }
    }

    pub async fn get_all_key_values(&self) -> Result<KeyValues, Exception> {
        let url = format!("{}?label=*", self.endpoints.get_uri(EndpointUrl::KeyValues)).parse::<Url>()?;

        Ok(self.get_request(url, Body::empty()).await?)
    }

    pub async fn get_key_values<S: Into<String>>(
        &self,
        key: S,
        label: Option<S>,
    ) -> Result<KeyValue, Exception> {
        let label_content = match label {
            Some(v) => v.into(),
            None => String::new(),
        };

        let url = format!(
            "{host}/{key}?label={label}",
            host = self.endpoints.get_uri(EndpointUrl::KeyValues),
            key = key.into(),
            label = label_content,
        )
        .parse::<Url>()?;

        Ok(self.get_request::<KeyValue>(url, Body::empty()).await?)
    }

    async fn get_request<T: DeserializeOwned>(&self, url: Url, body : Body) -> Result<T, Exception> {

        let req = create_signed_request(
            self.access_key.clone(),
            self.secret.clone(),
            url,
            body,
            Method::GET,
        )
        .await?;

        let mut result = req.await?;
        let json = result.body_string().await?;

        Ok(serde_json::from_str::<T>(&json)?)
    }

    pub fn endpoint_uri(&self) -> String {
        self.endpoints.base_endpoint()
    }
}


pub(crate) struct Body {
    contents: Vec<u8>
}

impl Body {

    fn empty() -> Self {
        Body {
            contents: vec![]
        }
    }
    pub(crate) fn value(self) -> Vec<u8> {
        self.contents
    }
}

impl From<Vec<u8>> for Body {
    fn from(bytes: Vec<u8>) -> Self {
        Body {
            contents: bytes.to_vec()
        }
    }
}

