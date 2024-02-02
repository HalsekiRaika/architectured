use uuid::Uuid;
use crate::command::Publication;
use crate::entities::PersonName;
use crate::error::KernelError;
use crate::event::PersonManipulationEvent;

#[derive(Debug, Clone)]
pub enum PersonManipulationCommand {
    Create { name: String },
    Rename { id: Uuid, name: String }
}

impl Publication<PersonManipulationEvent> for PersonManipulationCommand {
    type Error = KernelError;
    fn publish(self) -> Result<PersonManipulationEvent, Self::Error> {
        Ok(match self {
            PersonManipulationCommand::Create { name } => {
                PersonManipulationEvent::Created {
                    id: Default::default(),
                    name: PersonName::new(name),
                }
            }
            PersonManipulationCommand::Rename { name, .. } => {
                PersonManipulationEvent::Renamed {
                    name: PersonName::new(name),
                }
            }
        })
    }
}