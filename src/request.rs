use crate::Exception;
use chrono::{DateTime, Utc};
use futures::io::AsyncReadExt;
use futures::{AsyncRead, TryFutureExt};
use hmac::{Hmac, Mac};
use http::Method;
use sha2::{Digest, Sha256};
use std::borrow::Borrow;
use surf::middleware::HttpClient;

type HmacSha256 = Hmac<Sha256>;

pub(crate) struct Request;

const signedHeaders: &str = "date;host;x-ms-content-sha256";

impl Request {
    pub(crate) async fn sign<S: Into<String>>(
        secret: S,
        req: &mut http::Request<&[u8]>,
    ) -> Result<(), Exception> {
        let host = req.uri().host().unwrap().to_owned();
        let path = req.uri().path_and_query().unwrap().to_owned();
        let verb = req.method().to_string().to_uppercase();
        let contents = req.body_mut().to_vec();
        let utc = Utc::now().to_string();

        let mut hasher = Sha256::new();
        hasher.input(contents);
        let hashed_content = hasher.result();

        let content_hash = base64::encode(hashed_content.as_slice());

        let to_sign = format!("{}\n{}\n{};{};{}", verb, path, utc, host, content_hash);

        let mut mac = HmacSha256::new_varkey(secret.into().as_bytes()).expect("HMAC can take key of any size");

        mac.input(to_sign.as_bytes());

        //var stringToSign = $"{verb}\n{request.RequestUri.PathAndQuery}\n{utcNow.ToString("r")};{host};{contentHash}";

        //        using (var hmac = new HMACSHA256(secret))
        //        {
        //            signature = Convert.ToBase64String(hmac.ComputeHash(Encoding.ASCII.GetBytes(stringToSign)));
        //        }

        Ok(())
    }
}
