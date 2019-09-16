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

        //Create a key value
        let _kv = app_config_client
            .set_key("EnableHttps", "true" , SearchLabel::All, None, None)
            .await
            .unwrap();

        //Remove the key value
        app_config_client
            .remove_key_value("EnableHttps", SearchLabel::All)
            .await
            .unwrap();

        //Create a key with a label
        let _kv = app_config_client
            .set_key("EnableProxy", "true" , SearchLabel::For("ApplicationLabel"), None, None)
            .await
            .unwrap();

        //Remove the key with label
        app_config_client
            .remove_key_value("EnableProxy", SearchLabel::For("ApplicationLabel"))
            .await
            .unwrap();


    })
}
