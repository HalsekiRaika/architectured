use uuid::Uuid;
use crate::command::Publish;
use crate::entities::{BookId, Stock, Title};
use crate::error::KernelError;
use crate::event::BookEvent;

#[derive(Debug, Clone)]
pub enum BookCommand {
    Arrive { title: String, stock: i32 },
    Discard { id: Uuid, stock: i32 }
}

impl Publish<BookEvent> for BookCommand {
    type Error = KernelError;
    fn publish(self) -> Result<BookEvent, Self::Error> {
        Ok(match self {
            BookCommand::Arrive { title, stock } => {
                BookEvent::Arrival {
                    id: Default::default(),
                    title: Title::new(title),
                    stock: Stock::new(stock)?,
                }
            }
            BookCommand::Discard { id, stock } => {
                BookEvent::Discarded {
                    id: BookId::new(id),
                    stock: Stock::new(stock)?,
                }
            }
        })
    }
}