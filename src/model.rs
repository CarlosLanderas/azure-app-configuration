use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Key {
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Keys {
    items: Vec<Key>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Label {
    name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Labels {
    pub items: Vec<Label>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KeyValues {
    pub items: Vec<KeyValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct KeyValue {
    #[serde(skip_serializing)]
    pub etag: String,
    #[serde(skip_serializing)]
    pub key: String,
    #[serde(skip_serializing)]
    pub label: Option<String>,
    pub value: String,
    #[serde(skip_serializing)]
    pub content_type: Option<String>,
    #[serde(skip_serializing)]
    pub last_modified: String,
    #[serde(skip_serializing)]
    pub locked: bool,
    pub tags: HashMap<String, String>,
}
