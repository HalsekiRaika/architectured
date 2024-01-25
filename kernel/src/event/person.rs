use serde::{Deserialize, Serialize};
use crate::entities::{Person, PersonId, PersonName};
use crate::event::Applier;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PersonManipulationEvent {
    Created {
        id: PersonId,
        name: PersonName,
    },
    Renamed {
        name: PersonName,
    },
}

impl Applier<PersonManipulationEvent> for Person {
    fn apply(&mut self, event: PersonManipulationEvent) {
        match event {
            PersonManipulationEvent::Created { id, name } => {
                self.substitute(|person| {
                    *person.id = id;
                    *person.name = name;
                })
            },
            PersonManipulationEvent::Renamed { name } => {
                self.substitute(|person| {
                    *person.name = name;
                })
            },
        }
    }
}