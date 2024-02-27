use uuid::Uuid;
use crate::command::Publish;
use crate::entities::{BookId, PersonName};
use crate::error::KernelError;
use crate::event::{BookEvent, PersonEvent};

#[derive(Debug, Clone)]
pub enum PersonManipulationCommand {
    Create { name: String },
    Rename { id: Uuid, name: String },
    Rental { id: Uuid, book: Uuid },
    Return { id: Uuid, book: Uuid }
}

impl Publish<PersonEvent> for PersonManipulationCommand {
    type Error = KernelError;
    fn publish(self) -> Result<PersonEvent, Self::Error> {
        Ok(match self {
            PersonManipulationCommand::Create { name } => {
                PersonEvent::Created {
                    id: Default::default(),
                    name: PersonName::new(name),
                }
            }
            PersonManipulationCommand::Rename { name, .. } => {
                PersonEvent::Renamed {
                    name: PersonName::new(name),
                }
            }
            _ => return Err(KernelError::EventPublish)
        })
    }
}

impl Publish<(PersonEvent, BookEvent)> for PersonManipulationCommand {
    type Error = KernelError;
    
    fn publish(self) -> Result<(PersonEvent, BookEvent), Self::Error> {
        Ok(match self { 
            Self::Rental { book, .. } => {
                let p = PersonEvent::Rented {
                    id: BookId::new(book)
                };
                let b = BookEvent::Rental {
                    id: BookId::new(book),
                    rented_at: Default::default(),
                    rental_term: Default::default(),
                };
                (p, b)
            },
            Self::Return { book, .. } => {
                let p = PersonEvent::Returned {
                    id: BookId::new(book),
                };
                let b = BookEvent::Return {
                    id: BookId::new(book),
                };
                (p, b)
            }
            _ => return Err(KernelError::EventPublish)
        })
    }
}