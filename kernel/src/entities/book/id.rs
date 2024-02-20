use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct BookId(Uuid);

impl BookId {
    pub fn new(id: impl Into<Uuid>) -> BookId {
        Self(id.into())
    }
}

impl From<BookId> for Uuid {
    fn from(value: BookId) -> Self {
        value.0
    }
}

impl AsRef<Uuid> for BookId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Default for BookId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

