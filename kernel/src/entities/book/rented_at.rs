use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct RentedAt(OffsetDateTime);

impl RentedAt {
    pub fn new(at: impl Into<OffsetDateTime>) -> Self {
        Self(at.into())
    }
}

impl From<RentedAt> for OffsetDateTime {
    fn from(at: RentedAt) -> Self {
        at.0
    }
}

impl AsRef<OffsetDateTime> for RentedAt {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}

impl Default for RentedAt {
    fn default() -> Self {
        Self(OffsetDateTime::now_utc())
    }
}