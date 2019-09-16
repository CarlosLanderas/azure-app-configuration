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
