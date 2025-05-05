#[derive(Debug, Clone)]
pub enum Query {
    Accept(String),
    Deny(String),
}

impl Query {
    pub fn matches(&self, tags: &[String]) -> bool {
        match self {
            Query::Accept(s) => tags.contains(s),
            Query::Deny(s) => !tags.contains(s),
        }
    }
}

impl From<String> for Query {
    fn from(value: String) -> Self {
        if value.starts_with("-") {
            Query::Deny(value.strip_prefix("-").unwrap_or_default().to_string())
        } else {
            Query::Accept(value)
        }
    }
}
