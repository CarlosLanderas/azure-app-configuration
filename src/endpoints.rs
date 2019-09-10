use url::Url;

pub(crate) enum EndpointUrl {
    KeyValues,
    Keys,
}
pub(crate) const KEY_VALUE_ENDPOINT: &str = "kv";
pub(crate) const KEYS_ENDPOINT: &str = "keys";

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
        };

        e.parse::<Url>().unwrap()
    }
}
