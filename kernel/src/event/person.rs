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


#[cfg(test)]
mod test {
    use error_stack::{Report, ResultExt};
    use crate::entities::{PersonId, PersonName};
    use crate::error::test::AnyKernelError;
    use crate::event::PersonManipulationEvent;
    
    #[test]
    fn serialize() -> Result<(), Report<AnyKernelError>> {
        let ev = PersonManipulationEvent::Created {
            id: PersonId::default(),
            name: PersonName::new("test_man")
        };
        let se = serde_json::to_string(&ev)
            .change_context_lazy(|| AnyKernelError)?;
        
        dbg!(se);
        Ok(())
    }
}