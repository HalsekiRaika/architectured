use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PersonId(Uuid);

impl PersonId {
    pub fn new(id: impl Into<Uuid>) -> Self {
        Self(id.into())
    }
}

impl From<PersonId> for Uuid {
    fn from(value: PersonId) -> Self {
        value.0
    }
}

impl AsRef<Uuid> for PersonId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<PersonId> for String {
    fn from(value: PersonId) -> String {
        format!("{}", value.0)
    }
}

impl Default for PersonId {
    fn default() -> Self {
        Self::new(Uuid::new_v4())
    }
}