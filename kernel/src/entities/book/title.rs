use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Title(String);

impl Title {
    pub fn new(title: impl Into<String>) -> Title {
        Self(title.into())
    }
}

impl From<Title> for String {
    fn from(value: Title) -> Self {
        value.0
    }
}

impl AsRef<str> for Title {
    fn as_ref(&self) -> &str {
        &self.0
    }
}