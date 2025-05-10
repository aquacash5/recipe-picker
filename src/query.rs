use itertools::Itertools;
use unicase::UniCase;

#[derive(Debug, Clone)]
pub enum Query {
    Accept(UniCase<String>),
    Deny(UniCase<String>),
}

impl Query {
    pub fn matches(&self, tags: &[UniCase<String>]) -> bool {
        match self {
            Query::Accept(s) => tags.contains(s),
            Query::Deny(s) => !tags.contains(s),
        }
    }
}

impl From<String> for Query {
    fn from(value: String) -> Self {
        if value.starts_with("-") {
            Query::Deny(UniCase::new(
                value
                    .strip_prefix("-")
                    .unwrap_or_default()
                    .split("_")
                    .join(" "),
            ))
        } else {
            Query::Accept(UniCase::new(value.split("_").join(" ")))
        }
    }
}
