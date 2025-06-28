use itertools::Itertools;
use unicase::UniCase;

#[derive(Debug, Clone)]
pub enum Query {
    Accept(String),
    Deny(String),
}

impl Query {
    pub fn matches(&self, tags: &[impl AsRef<str>]) -> bool {
        match self {
            Query::Accept(s) => tags
                .iter()
                .map(AsRef::<str>::as_ref)
                .map(UniCase::unicode)
                .contains(&UniCase::new(s.as_str())),
            Query::Deny(s) => !tags
                .iter()
                .map(AsRef::<str>::as_ref)
                .map(UniCase::unicode)
                .contains(&UniCase::new(s.as_str())),
        }
    }
}

impl From<String> for Query {
    fn from(value: String) -> Self {
        if value.starts_with("-") {
            Query::Deny(
                value
                    .strip_prefix("-")
                    .unwrap_or(&value)
                    .split("_")
                    .join(" "),
            )
        } else {
            Query::Accept(value.split("_").join(" "))
        }
    }
}
