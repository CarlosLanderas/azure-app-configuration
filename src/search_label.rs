use std::string::ToString;

pub enum SearchLabel<'a> {
    All,
    For(&'a str),
}

impl<'a> ToString for SearchLabel<'a> {
    fn to_string(&self) -> String {
        match self {
            SearchLabel::All => String::new(),
            SearchLabel::For(v) => v.to_string(),
        }
    }
}

#[test]
fn searchlabel_test() {
    assert_eq!(SearchLabel::All.to_string(), String::new());
    assert_eq!(SearchLabel::For("Label1").to_string(), "Label1");
    assert_eq!(SearchLabel::For("Superlabel").to_string(), "Superlabel");
}
