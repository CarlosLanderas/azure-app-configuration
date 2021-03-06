use async_std::task;
use azure_app_configuration::client::AzureAppConfigClient;
use azure_app_configuration::search_label::SearchLabel;

fn main() {
    task::block_on(async {
        let app_config_client = AzureAppConfigClient::new(
            "https://lande-app-configuration.azconfig.io",
            "0-l9-s0:Z6DMwn2DoiKxgVsTIm7h",
            "wgf9BDWeh/+Dtq8DmpsJSUpwrdgYLrXG8svE+VyM06w=",
        );

        //List all key values without a label (all key values);
        let key_values = app_config_client.list_key_values(SearchLabel::All).await;
        for k in key_values {
            println!("{:?}", k);
        }
    })
}
