use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Key {
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Keys {
    items: Vec<Key>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Label {
    name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Labels {
    items: Vec<Label>
}


#[derive(Deserialize, Debug, Clone)]
pub struct KeyValues {
    items: Vec<KeyValue>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KeyValue {
    pub etag: String,
    pub key: String,
    pub label: Option<String>,
    pub value: String,
    pub content_type: String,
    pub last_modified: String,
    pub locked: bool,
    pub tags: HashMap<String, String>,
}
