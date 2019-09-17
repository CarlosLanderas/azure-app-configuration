use async_std::task;
use azure_app_configuration::client::AzureAppConfigClient;

fn main() {
    task::block_on(async {
        let app_config_client = AzureAppConfigClient::new(
            "https://lande-app-configuration.azconfig.io",
            "0-l9-s0:Z6DMwn2DoiKxgVsTIm7h",
            "wgf9BDWeh/+Dtq8DmpsJSUpwrdgYLrXG8svE+VyM06w=",
        );

        //List all key values without a label (all key values);
        let keys = app_config_client.list_keys().await.unwrap();
        for k in keys.items {
            println!("{:?}", k);
        }
    })
}
