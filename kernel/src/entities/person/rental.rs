use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::entities::BookId;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rental(HashSet<BookId>);

impl Rental {
    pub fn new(rental: impl Into<HashSet<BookId>>) -> Rental {
        Self(rental.into())
    }
}

