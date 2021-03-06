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

        //Retrieve a key called ConnectionString for label ConsotoApp
        let mut kv = app_config_client
            .get_key_value("ConnectionString", SearchLabel::For("ContosoApp"))
            .await;

        println!("{:?}", kv);

        //Retrieve the key ConnectionString that has not label associated

        kv = app_config_client
            .get_key_value("ConnectionString", SearchLabel::All)
            .await;

        println!("{:?}", kv);
    })
}
