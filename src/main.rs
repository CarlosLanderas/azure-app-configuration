use http::Method;
use url::Url;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use async_std::task;
use rust_azure_app_config::client::AzureAppConfigClient;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    task::block_on(async {
        let az = AzureAppConfigClient::new(
            "https://lande-app-configuration.azconfig.io",
            "0-l9-s0:Z6DMwn2DoiKxgVsTIm7h",
            base64::decode("wgf9BDWeh/+Dtq8DmpsJSUpwrdgYLrXG8svE+VyM06w=").unwrap(),
        );

        let mut tags = HashMap::new();
        tags.insert("tag1".to_owned(), "tagvalue1".to_owned());
        tags.insert("tag2".to_owned(), "tagvalue2".to_owned());

        let r = az.set_key("Bob", "Lucas", None, Some(tags), None).await;

        println!("{:?}", r);

//        let a = az.list_labels().await.unwrap();
//        println!("{:?}", a);
//                let result = az
//                    .get_key_value("HealthChecksUI:HealthChecks:0:Uri", Some("HealthChecksConfig"))
//                    .await
//                    .unwrap();
//                println!("{:?}", result);
    })
}
