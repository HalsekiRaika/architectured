use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct RentalTerm(OffsetDateTime);

impl RentalTerm {
    pub fn new(at: impl Into<OffsetDateTime>) -> Self {
        Self(at.into())
    }
}

impl From<RentalTerm> for OffsetDateTime {
    fn from(at: RentalTerm) -> Self {
        at.0
    }
}

impl AsRef<OffsetDateTime> for RentalTerm {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}

impl Default for RentalTerm {
    fn default() -> Self {
        Self(OffsetDateTime::now_utc())
    }
}