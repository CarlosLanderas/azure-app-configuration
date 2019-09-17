# Azure App Configuration client for Rust

[![Build Status](https://travis-ci.org/CarlosLanderas/azure-app-configuration.svg?branch=master)](https://travis-ci.org/CarlosLanderas/azure-app-configuration)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/CarlosLanderas/azure-app-configuration)
[![Cargo](https://img.shields.io/crates/v/azure-app-configuration.svg)](https://crates.io/crates/azure-app-configuration)
[![Documentation](https://docs.rs/azure-app-configuration/badge.svg)](https://docs.rs/azure-app-configuration)

With **azure-app-configuration** you can easily work with your Azure App Configuration service centralized configurations.

Latest version supports:

- List keys
- List labels
- List key values
- Get key value
- Set key value (with label, tags and content type)
- Remove key value

## Running samples

You can find some sample code snippets here: [examples](https://github.com/CarlosLanderas/azure-app-configuration/tree/master/examples)
just replace the configuration with your Azure configuration endpoint, your access_key and secret and execute them by using:

`cargo run --example list-key-values`

`cargo run --example get_key_value`

`cargo run --example set_key_value`

You can see all available targets in **[Cargo.toml](https://github.com/CarlosLanderas/azure-app-configuration/blob/master/Cargo.toml#L25-L42)** file

## Code samples

### Create an AzureAppConfiguration client

To create an Azure App Configuration client just use ::new method and provide the endpoint url, the access key and the secret:

```rust
use azure_app_configuration::client::AzureAppConfigClient;
let app_config_client = AzureAppConfigClient::new(
        "https://endpoint.azconfig.io",
        "0-l9-s0:Z6DMwn2DoiK2gVsTIm7h",
        "wgf9BDWeh/+Dtq8Dmps3SUpwrdgYLrXG8svE+VyM06w=",
        );
```

### List keys

```rust
   //List all key values without a label (all key values);
    let keys = app_config_client.list_keys().await.unwrap();
    for k in keys.items {
        println!("{:?}", k);
    }
```

### List labels

```rust
  let labels = app_config_client.list_labels().await.unwrap();
  for l in labels.items {
    println!("{:?}", l);
  }
```

### List Key Values

```rust
    let key_values = app_config_client.list_key_values(SearchLabel::All).await;
    for k in key_values {
        println!("{:?}", k);
    }
```

### Get Key value with label

Retrieve value for key ConnectionString using label ContosoApp

```rust
   let kv = app_config_client
            .get_key_value("ConnectionString", SearchLabel::For("ContosoApp"))
            .await;
   println!("{:?}", kv);
```

### Get Key value without label

Retrieve a label called ConnectionString with no label specified

```rust
   let kv = app_config_client
            .get_key_value("ConnectionString", SearchLabel::All)
            .await;
   println!("{:?}", kv);
```

### Set key value

Delete ConnectionString key for Application1 label

```rust
    let kv = app_config_client
            .set_key(
                "ConnectionString",
                "Server=dummy;user id=user;password=fakepass",
                SearchLabel::For("Application1"),
                None,
                None,
            )
            .await;

        println!("{:?}", kv);
```

### Set key value with tags and content type

```rust
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
```