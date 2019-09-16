use crate::endpoints::{EndpointUrl, Endpoints};
use crate::error::HttpError;
use crate::model::{KeyValue, KeyValues, Keys, Labels};
use crate::request_sign::create_signed_request;
use crate::search_label::SearchLabel;
use crate::Exception;
use http::Method;
use serde::de::DeserializeOwned;
use std::str::FromStr;

use mime::Mime;
use std::collections::HashMap;
use url::Url;

const APP_CONFIG_MIME: &str = "application/vnd.microsoft.appconfig.kv+json";

pub struct AzureAppConfigClient {
    access_key: String,
    secret: Vec<u8>,
    endpoints: Endpoints,
}

impl AzureAppConfigClient {
    pub fn new<S: Into<String>>(uri_endpoint: S, access_key: S, secret: S) -> AzureAppConfigClient {
        AzureAppConfigClient {
            access_key: access_key.into(),
            secret: base64::decode(&secret.into()).expect("Could not decode secret key"),
            endpoints: Endpoints::new(uri_endpoint.into()),
        }
    }

    pub async fn list_labels(&self) -> Result<Labels, Exception> {
        let url = &format!("{}", self.endpoints.get_uri(EndpointUrl::Labels)).parse::<Url>()?;
        Ok(self.send_json(url, Method::GET, Body::empty()).await?)
    }

    pub async fn list_keys(&self) -> Result<Keys, Exception> {
        let url = &format!("{}", self.endpoints.get_uri(EndpointUrl::Keys)).parse::<Url>()?;
        Ok(self.send_json(url, Method::GET, Body::empty()).await?)
    }

    pub async fn list_key_values<'a>(
        &self,
        label: SearchLabel<'a>,
    ) -> Result<KeyValues, Exception> {
        let url = &format!(
            "{}?label={}",
            self.endpoints.get_uri(EndpointUrl::KeyValues),
            label.to_string()
        )
        .parse::<Url>()?;

        Ok(self.send_json(url, Method::GET, Body::empty()).await?)
    }

    pub async fn set_key<'a, S: Into<String>>(
        &self,
        key: S,
        value: S,
        label: SearchLabel<'a>,
        tags: Option<HashMap<S, S>>,
        content_type: Option<S>,
    ) -> Result<KeyValue, Exception> {
        let mut k = KeyValue::default();

        k.value = value.into();

        k.content_type = match content_type {
            Some(c) => Some(c.into()),
            None => None,
        };

        if let Some(tg) = tags {
            for (ky, v) in tg {
                k.tags.insert(ky.into(), v.into());
            }
        }

        let json = serde_json::to_string(&k)?;

        let url = &format!(
            "{}/{key}?label={lbl}",
            self.endpoints.get_uri(EndpointUrl::KeyValues),
            key = key.into(),
            lbl = label.to_string()
        )
        .parse::<Url>()?;

        Ok(self
            .send_json(url, Method::PUT, Body::from(json.into_bytes()))
            .await?)
    }

    pub async fn get_key_value<'a, S: Into<String>>(
        &self,
        key: S,
        label: SearchLabel<'a>,
    ) -> Result<KeyValue, Exception> {
        let url = &format!(
            "{host}/{key}?label={label}",
            host = self.endpoints.get_uri(EndpointUrl::KeyValues),
            key = key.into(),
            label = label.to_string(),
        )
        .parse::<Url>()?;

        Ok(self
            .send_json::<KeyValue>(url, Method::GET, Body::empty())
            .await?)
    }

    pub async fn remove_key_value<'a, S: Into<String>>(
        &self,
        key: S,
        label: SearchLabel<'a>,
    ) -> Result<(), Exception> {
        let url = get_key_value_url(self, key, label)?;

        self.send_request(&url, Method::DELETE, Body::empty())
            .await?;

        Ok(())
    }

    async fn send_request(
        &self,
        url: &Url,
        method: Method,
        body: Body,
    ) -> Result<String, Exception> {
        log::debug!(
            "Sending {} request to {}",
            &method.to_string(),
            &url.to_string()
        );

        let mut req = create_signed_request(
            self.access_key.clone(),
            self.secret.clone(),
            url,
            body,
            method.clone(),
        )
        .await?;

        if method != Method::GET {
            req = req.set_mime(Mime::from_str(APP_CONFIG_MIME).unwrap());
        }

        let mut result = req.await?;
        let content = result.body_string().await?;

        match result.status() {
            v if !v.is_success() => Err(HttpError::new(v.as_u16() as usize, url.as_str()).into()),
            _ => Ok(content),
        }
    }
    async fn send_json<T: DeserializeOwned>(
        &self,
        url: &Url,
        method: Method,
        body: Body,
    ) -> Result<T, Exception> {
        let result = self.send_request(url, method, body).await?;

        log::debug!("JSON: {}", result);

        Ok(serde_json::from_str::<T>(&result)?)
    }

    pub fn endpoint_uri(&self) -> String {
        self.endpoints.base_endpoint()
    }
}

fn get_key_value_url<S: Into<String>>(
    client: &AzureAppConfigClient,
    key: S,
    label: SearchLabel,
) -> Result<Url, Exception> {
    format!(
        "{host}/{key}?label={label}",
        host = client.endpoints.get_uri(EndpointUrl::KeyValues),
        key = key.into(),
        label = label.to_string(),
    )
    .parse::<Url>()
    .map_err(|e| e.into())
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
