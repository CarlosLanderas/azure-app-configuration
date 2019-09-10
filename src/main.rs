use http::Method;
use url::Url;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use async_std::task;
use rust_azure_app_config::client::AzureAppConfigClient;
use std::str::FromStr;

fn main() {
    task::block_on(async {
        let az = AzureAppConfigClient::new(
            "https://lande-app-configuration.azconfig.io",
            "0-l9-s0:Z6DMwn2DoiKxgVsTIm7h",
            base64::decode("wgf9BDWeh/+Dtq8DmpsJSUpwrdgYLrXG8svE+VyM06w=").unwrap(),
        );

        let a = az.list_keys().await.unwrap();
        println!("{:?}", a);
//        let result = az
//            .get_key_values("HealthChecksUI:EvaluationTimeOnSeconds", None)
//            .await
//            .unwrap();
//        println!("{:?}", result);
    })
}
