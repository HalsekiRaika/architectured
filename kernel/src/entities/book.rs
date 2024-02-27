mod id;
mod title;
mod stock;
mod rented_at;
mod rental_term;

pub use self::id::*;
pub use self::title::*;
pub use self::stock::*;
pub use self::rented_at::*;
pub use self::rental_term::*;

use destructure::{Destructure, Mutation};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Default, Deserialize, Serialize, Destructure, Mutation)]
pub struct Book {
    id: BookId,
    title: Title,
    stock: Stock
}

impl Book {
    pub fn new(
        id: BookId,
        title: Title,
        stock: Stock
    ) -> Book {
        Self {
            id,
            title,
            stock
        }
    }
}

impl Book {
    pub fn id(&self) -> &BookId {
        &self.id
    }
    pub fn title(&self) -> &Title {
        &self.title
    }
    pub fn stock(&self) -> &Stock {
        &self.stock
    }
}