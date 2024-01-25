use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PersonName(String);

impl PersonName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl From<PersonName> for String {
    fn from(value: PersonName) -> Self {
        value.0
    }
}

impl AsRef<str> for PersonName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for PersonName {
    fn default() -> Self {
        Self::new(String::new())
    }
}