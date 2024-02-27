use serde::{Deserialize, Serialize};
use crate::entities::{Book, BookId, RentalTerm, RentedAt, Stock, Title};
use crate::event::Applier;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BookEvent {
    Arrival { id: BookId, title: Title, stock: Stock },
    Discarded { id: BookId, stock: Stock },
    Rental { id: BookId, rented_at: RentedAt, rental_term: RentalTerm },
    Return { id: BookId },
}

impl Applier<BookEvent> for Book {
    fn apply(&mut self, event: BookEvent) {
        match event {
            BookEvent::Arrival { id, title, stock } => {
                self.substitute(|book| {
                    *book.id = id;
                    *book.title = title;
                    *book.stock = stock;
                })
            }
            BookEvent::Discarded { id, .. } => {
                self.substitute(|book| {
                    *book.id = id;
                    book.stock.apply(event);
                })
            }
            BookEvent::Rental { id, .. } => {
                self.substitute(|book| {
                    *book.id = id;
                    book.stock.apply(event);
                })
            }
            BookEvent::Return { id } => {
                self.substitute(|book| {
                    *book.id = id;
                    book.stock.apply(event);
                })
            }
        }
    }
}