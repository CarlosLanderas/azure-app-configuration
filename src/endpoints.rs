use url::Url;

pub(crate) enum EndpointUrl {
    KeyValues,
    Keys,
    Labels,
}
pub(crate) const KEY_VALUE_ENDPOINT: &str = "kv";
pub(crate) const KEYS_ENDPOINT: &str = "keys";
pub(crate) const LABELS_ENDPOINT: &str = "labels";

pub(crate) struct Endpoints {
    base_endpoint: String,
}

impl Endpoints {
    pub(crate) fn new<S: Into<String>>(config_endpoint: S) -> Endpoints {
        Endpoints {
            base_endpoint: config_endpoint.into(),
        }
    }

    pub(crate) fn base_endpoint(&self) -> String {
        self.base_endpoint.clone()
    }

    pub(crate) fn get_uri(&self, endpoint: EndpointUrl) -> Url {
        let e = match endpoint {
            EndpointUrl::KeyValues => format!("{}/{}", self.base_endpoint, KEY_VALUE_ENDPOINT),
            EndpointUrl::Keys => format!("{}/{}", self.base_endpoint, KEYS_ENDPOINT),
            EndpointUrl::Labels => format!("{}/{}", self.base_endpoint, LABELS_ENDPOINT),
        };

        e.parse::<Url>().unwrap()
    }
}

#[test]
fn endpoints_url_test() {
    let endpoints = Endpoints::new("http://sample.io");
    assert_eq!(
        endpoints.get_uri(EndpointUrl::Keys).to_string(),
        "http://sample.io/keys"
    );
    assert_eq!(
        endpoints.get_uri(EndpointUrl::KeyValues).to_string(),
        "http://sample.io/kv"
    );
    assert_eq!(
        endpoints.get_uri(EndpointUrl::Labels).to_string(),
        "http://sample.io/labels"
    );
}
