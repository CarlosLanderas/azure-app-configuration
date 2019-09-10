use crate::endpoints::{EndpointUrl, Endpoints};
use crate::model::{Key, KeyValue, KeyValues, Keys, Labels};
use crate::request_sign::create_signed_request;
use crate::requests::KeyValueRequest;
use crate::Exception;
use http::{Method, Response, StatusCode, Uri};
use serde::de::DeserializeOwned;
use serde::export::fmt::Display;
use serde::Deserialize;
use std::error::Error;
use std::fmt::Formatter;
use std::str::FromStr;
use surf::middleware::HttpClient;
use url::Url;
use std::collections::HashMap;
use mime::Mime;


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

    pub async fn list_labels(&self) -> Result<Labels, Exception> {
        let url = &format!("{}", self.endpoints.get_uri(EndpointUrl::Labels)).parse::<Url>()?;
        Ok(self.send_request(url, Method::GET, Body::empty()).await?)
    }

    pub async fn list_keys(&self) -> Result<Keys, Exception> {
        let url = &format!("{}", self.endpoints.get_uri(EndpointUrl::Keys)).parse::<Url>()?;
        Ok(self.send_request(url, Method::GET, Body::empty()).await?)
    }

    pub async fn list_key_values(&self) -> Result<KeyValues, Exception> {
        let url = &format!("{}?label=*", self.endpoints.get_uri(EndpointUrl::KeyValues))
            .parse::<Url>()?;

        Ok(self.send_request(url, Method::GET, Body::empty()).await?)
    }

    pub async fn set_key<S: Into<String>>(
        &self,
        key: S,
        value: S,
        label: Option<String>,
        tags: Option<HashMap<String,String>>,
        content_type: Option<String>,
    ) -> Result<KeyValue, Exception>  {

        let mut k = KeyValue::default();
        k.value = value.into();
        k.content_type = Some(content_type.unwrap_or(String::new()));

       if let Some(tg) = tags {
           for (ky, v) in tg {
                k.tags.insert(ky, v);
           }
       }
        let target_label  = label.unwrap_or(String::new()).to_string();

        let json = serde_json::to_string(&k)?;
        println!("{}", json);

        let url = &format!(
            "{}/{key}?label={lbl}",
            self.endpoints.get_uri(EndpointUrl::KeyValues),
            key = key.into(),
            lbl = target_label
        )
        .parse::<Url>()?;

        Ok(self.send_request(url, Method::PUT, Body::from(json.into_bytes())).await?)
    }

    pub async fn get_key_value<S: Into<String>>(
        &self,
        key: S,
        label: Option<S>,
    ) -> Result<KeyValue, Exception> {
        let label_content = match label {
            Some(v) => v.into(),
            None => String::new(),
        };

        let url = &format!(
            "{host}/{key}?label={label}",
            host = self.endpoints.get_uri(EndpointUrl::KeyValues),
            key = key.into(),
            label = label_content,
        )
        .parse::<Url>()?;

        Ok(self
            .send_request::<KeyValue>(url, Method::GET, Body::empty())
            .await?)
    }

    async fn send_request<T: DeserializeOwned>(
        &self,
        url: &Url,
        method: Method,
        body: Body,
    ) -> Result<T, Exception> {
        let mut req = create_signed_request(
            self.access_key.clone(),
            self.secret.clone(),
            url,
            body,
            method.clone(),
        )
        .await?;


        if method != Method::GET {
            req = req.set_mime(Mime::from_str("application/vnd.microsoft.appconfig.kv+json").unwrap());
        }

        let mut result = req.await?;
        let json = result.body_string().await?;

        match result.status() {
            v if v != http::StatusCode::OK => {
                Err(HttpError::new(v.as_u16() as usize, url.as_str()))
            }
            _ => Ok(()),
        }?;

        println!("{}", json);
        Ok(serde_json::from_str::<T>(&json)?)
    }

    pub fn endpoint_uri(&self) -> String {
        self.endpoints.base_endpoint()
    }
}

#[derive(Debug, Clone)]
struct HttpError {
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

pub(crate) struct Body {
    contents: Vec<u8>,
}

impl Body {
    fn empty() -> Self {
        Body { contents: vec![] }
    }
    pub(crate) fn value(&self) -> Vec<u8> {
        self.contents.clone()
    }
}

impl From<Vec<u8>> for Body {
    fn from(bytes: Vec<u8>) -> Self {
        Body {
            contents: bytes.to_vec(),
        }
    }
}
