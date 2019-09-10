use crate::Exception;
use async_std::io::Read;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use hmac::{Hmac, Mac};
use http::{Method, Response, Uri};
use sha2::{Digest, Sha256};
use std::convert::TryFrom;
use url::Url;
use surf::middleware::HttpClient;
use httpdate::fmt_http_date;

type HmacSha256 = Hmac<Sha256>;

pub(crate) struct Request;

const signedHeaders: &str = "date;host;x-ms-content-sha256";

impl<'a> Request {
    pub(crate) async fn create_signed_request<S: Into<String>>(
        access_key: S,
        secret: Vec<u8>,
        url: Url,
        body: Vec<u8>,
        method: Method,
    ) -> Result<surf::Request<impl HttpClient>, Exception> {

        let host = url.host().unwrap().to_string();
        let path = format!("{}?{}", url.path(), url.query().unwrap());
        let verb = method.to_string().to_uppercase();
        let utc = fmt_http_date(std::time::SystemTime::now());

        let mut hasher = Sha256::new();
        hasher.input(&body);
        let hashed_content = hasher.result();

        let content_hash = base64::encode(hashed_content.as_slice());

        let to_sign = format!("{}\n{}\n{};{};{}", verb, path, utc, host, content_hash);

        println!("{}", to_sign);

        let mut mac = HmacSha256::new_varkey(&secret).expect("HMAC can take key of any size");

        mac.input(to_sign.as_bytes());

        let result = mac.result().code();
        let encoded_signature = base64::encode(&result);

        let mut request = surf::Request::new(method, url.clone());
        let mut h = request.headers();

        h.insert("Date", utc);

        h.insert("x-ms-content-sha256", content_hash);

        let auth_value = format!(
            "HMAC-SHA256 Credential={}&SignedHeaders={}&Signature={}",
            access_key.into(),
            signedHeaders,
            encoded_signature
        );

        h.insert("Authorization", auth_value);
        h.insert("host", url.host().unwrap().to_string());

        println!("{}", request.header("Date").unwrap());
        println!("{}", request.header("Authorization").unwrap());
        println!("{}", request.header("x-ms-content-sha256").unwrap());

        Ok(request)
    }
}
