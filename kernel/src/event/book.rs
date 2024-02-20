use serde::{Deserialize, Serialize};
use crate::entities::{BookId, RentalTerm, RentedAt};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BookEvent {
    Rental { id: BookId, rented_at: RentedAt, rental_term: RentalTerm },
    Return { id: BookId },
}