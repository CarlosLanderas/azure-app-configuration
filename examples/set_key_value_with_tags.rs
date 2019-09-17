use async_std::task;
use azure_app_configuration::client::AzureAppConfigClient;
use azure_app_configuration::search_label::SearchLabel;
use std::collections::HashMap;

fn main() {
    task::block_on(async {
        let app_config_client = AzureAppConfigClient::new(
            "https://lande-app-configuration.azconfig.io",
            "0-l9-s0:Z6DMwn2DoiKxgVsTIm7h",
            "wgf9BDWeh/+Dtq8DmpsJSUpwrdgYLrXG8svE+VyM06w=",
        );

        let mut tags = HashMap::new();
        tags.insert("tag1", "tagvalue1");
        tags.insert("tag2", "tagvalue2");

        let kv = app_config_client
            .set_key(
                "UseCache",
                "true",
                SearchLabel::For("PublicWebsite"),
                Some(tags),
                Some("application/lande"),
            )
            .await;

        println!("{:?}", kv);
    })
}
