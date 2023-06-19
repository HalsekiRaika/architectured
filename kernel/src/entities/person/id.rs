use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PersonId(String);

impl PersonId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<PersonId> for String {
    fn from(value: PersonId) -> Self {
        value.0
    }
}

impl AsRef<str> for PersonId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for PersonId {
    fn default() -> Self {
        use rand::distributions::{Alphanumeric, Distribution};
        let gen = Alphanumeric.sample_iter(&mut rand::thread_rng())
            .take(16)
            .map(char::from)
            .collect::<String>();
        Self::new(gen)
    }
}