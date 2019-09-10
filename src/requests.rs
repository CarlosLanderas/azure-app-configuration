use std::collections::HashMap;

pub struct KeyValueRequest {
    key: String,
    label: String,
    conditions: HashMap<String, String>,
}

impl KeyValueRequest {
    pub fn new() -> KeyValueRequest {
        KeyValueRequest::default()
    }

    pub fn with_key<S: Into<String>>(&mut self, key: S) -> &mut Self {
        self.key = key.into();
        self
    }

    pub fn with_label<S: Into<String>>(&mut self, label: S) -> &mut Self {
        self.label = label.into();
        self
    }

    pub fn with_all_labels<S: Into<String>>(&mut self) -> &mut Self {
        self.label = "*".to_owned();
        self
    }
}

impl Default for KeyValueRequest {
    fn default() -> Self {
        KeyValueRequest {
            label: "*".to_owned(),
            key: String::new(),
            conditions: HashMap::new(),
        }
    }
}
