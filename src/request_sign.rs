use crate::client::Body;
use crate::Exception;
use hmac::{Hmac, Mac};
use http::Method;
use httpdate::fmt_http_date;
use sha2::{Digest, Sha256};
use surf::middleware::HttpClient;
use url::Url;

type HmacSha256 = Hmac<Sha256>;

const SIGNED_HEADERS: &str = "date;host;x-ms-content-sha256";

fn get_content_hash_base64(body: &Body) -> String {
    let mut hasher = Sha256::new();
    hasher.input(body.value());

    let hashed_content = hasher.result();
    base64::encode(&hashed_content)
}

fn get_hmac(secret: Vec<u8>, to_sign: String) -> String {
    let mut mac = HmacSha256::new_varkey(&secret).expect("HMAC can take key of any size");
    mac.input(to_sign.as_bytes());

    let result = mac.result().code();
    base64::encode(&result)
}

pub(crate) async fn create_signed_request<S: Into<String>>(
    access_key: S,
    secret: Vec<u8>,
    url: &Url,
    body: Body,
    method: Method,
) -> Result<surf::Request<impl HttpClient>, Exception> {
    let host = url.host().unwrap().to_string();

    let path = match url.query() {
        Some(_) => format!("{}?{}", url.path(), url.query().unwrap()),
        None => url.path().to_string(),
    };

    let verb = method.to_string().to_uppercase();
    let utc = fmt_http_date(std::time::SystemTime::now());

    let content_hash = get_content_hash_base64(&body);

    let to_sign = format!("{}\n{}\n{};{};{}", verb, path, utc, host, content_hash);

    let encoded_signature = get_hmac(secret, to_sign);

    let mut request = surf::Request::new(method, url.clone());
    let mut h = request.headers();

    let auth_value = format!(
        "HMAC-SHA256 Credential={}&SignedHeaders={}&Signature={}",
        access_key.into(),
        SIGNED_HEADERS,
        encoded_signature
    );

    log::debug!(
        "Request signed with headers\n \
         Date: {}\n \
         x-ms-content-sha256: {}\n \
         Authorization: {}",
        &utc,
        &content_hash,
        &auth_value
    );

    log::debug!("Request body size: {}",  body.len());

    h.insert("Date", utc);
    h.insert("x-ms-content-sha256", content_hash);
    h.insert("Authorization", auth_value);
    h.insert("host", url.host().unwrap().to_string());

    request = request.body_bytes(body.value());

    Ok(request)
}
