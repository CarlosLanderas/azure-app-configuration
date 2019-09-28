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
    /// Creates a new instance of Azure App Configuration Client
    /// # Arguments
    ///
    /// * `uri_endpoint` - Your Azure App Configuration service url
    /// * `access_key` - Azure provided access_key
    /// * `secret` - Azure provided secret key
    ///
    /// # Examples
    /// ```
    /// use azure_app_configuration::client::AzureAppConfigClient;
    ///
    /// let client = AzureAppConfigClient::new(
    ///     "https://yourendpoint.azconfig.io",
    ///     "access_key",
    ///     "wgf9BDWeh/+Dtq8DmpsJSUpwrdgYLrXG8svE+VyM06w=");
    /// ```
    pub fn new<S: Into<String>>(uri_endpoint: S, access_key: S, secret: S) -> AzureAppConfigClient {
        AzureAppConfigClient {
            access_key: access_key.into(),
            secret: base64::decode(&secret.into()).expect("Could not decode secret key"),
            endpoints: Endpoints::new(uri_endpoint.into()),
        }
    }
    /// List all available labels in Azure App Configuration service
    /// # Examples
    /// ```no run
    ///  let labels = app_config_client.list_labels().await.unwrap();
    ///  for l in labels.items {
    ///      println!("{:?}", l);
    ///  }
    ///
    /// ```
    pub async fn list_labels(&self) -> Result<Labels, Exception> {
        let url = &format!("{}", self.endpoints.get_uri(EndpointUrl::Labels)).parse::<Url>()?;
        Ok(self.send_json(url, Method::GET, Body::empty()).await?)
    }

    /// List all available keys in Azure App Configuration service
    /// # Examples
    /// ```no run
    /// let keys = app_config_client.list_keys().await.unwrap();
    /// for k in keys.items {
    ///     println!("{:?}", k);
    /// }
    /// ```
    pub async fn list_keys(&self) -> Result<Keys, Exception> {
        let url = &format!("{}", self.endpoints.get_uri(EndpointUrl::Keys)).parse::<Url>()?;
        Ok(self.send_json(url, Method::GET, Body::empty()).await?)
    }

    /// List all available key values in Azure App Configuration service
    /// # Examples
    /// ```no run
    /// let key_values = app_config_client.list_key_values(SearchLabel::All).await;
    ///  for k in key_values {
    ///     println!("{:?}", k);
    ///  }
    /// ```
    pub async fn list_key_values(&self, label: SearchLabel<'_>) -> Result<KeyValues, Exception> {
        let url = &format!(
            "{}?label={}",
            self.endpoints.get_uri(EndpointUrl::KeyValues),
            label.to_string()
        )
        .parse::<Url>()?;

        Ok(self.send_json(url, Method::GET, Body::empty()).await?)
    }

    /// Set the target key with the desired value, label, tags and content-type
    /// # Arguments
    /// * `key` - Key name to be set
    /// * `value` - Key value
    /// * `label` - Key label (SearchLabel::All for no label and SearchLabel::For("label") to stablish label
    /// * `tags` - HashMap<String,String> collection of key value associated tags
    /// * `content_type` - Key associated content-type
    /// # Examples
    /// ```no run
    /// use std::collections::HashMap;
    /// use azure_app_configuration::search_label::SearchLabel;
    ///
    /// let mut tags = HashMap::new();
    ///        tags.insert("tag1", "tagvalue1");
    ///        tags.insert("tag2", "tagvalue2");
    ///
    ///        let kv = app_config_client
    ///            .set_key(
    ///                "UseCache",
    ///                "true",
    ///                SearchLabel::For("PublicWebsite"),
    ///                Some(tags),
    ///                None,
    ///            )
    ///            .await;
    ///        println!("{:?}", kv);
    ///
    /// ```
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

    /// Get key value
    /// # Arguments
    /// * `key` - Key name to be set
    /// * `label` - Key label (SearchLabel::All for no label and SearchLabel::For("label") to stablish label
    /// # Examples
    /// ```no run
    /// use azure_app_configuration::search_label::SearchLabel;
    ///
    /// let kv = app_config_client
    ///    .get_key_value("ConnectionString", SearchLabel::For("ContosoApp"))
    ///    .await;
    /// ```
    pub async fn get_key_value<S: Into<String>>(
        &self,
        key: S,
        label: SearchLabel<'_>,
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

    /// Remove target key value from Azure App Configuration service
    /// # Arguments
    /// * `key` - Key to be deleted
    /// * `label` - Label where the key will be found and removed, if no label is specified all labels with that key will be remove
    /// # Examples
    /// ```no run
    /// app_config_client
    ///    .remove_key_value("EnableProxy", SearchLabel::For("ApplicationLabel"))
    ///    .await
    ///    .unwrap();    ///
    /// ```
    pub async fn remove_key_value<S: Into<String>>(
        &self,
        key: S,
        label: SearchLabel<'_>,
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

#[derive(Debug, Clone)]
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
    pub(crate) fn len(&self) -> usize {
        self.contents.len()
    }
}

impl From<Vec<u8>> for Body {
    fn from(bytes: Vec<u8>) -> Self {
        Body {
            contents: bytes.to_vec(),
        }
    }
}

impl From<&str> for Body {
    fn from(value: &str) -> Self {
        Body::from(value.as_bytes().to_vec())
    }
}
