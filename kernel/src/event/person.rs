use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::entities::{BookId, Person, PersonId, PersonName};
use crate::event::Applier;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PersonEvent {
    Created {
        id: PersonId,
        name: PersonName,
    },
    Renamed {
        name: PersonName,
    },
    Rented {
        id: BookId
    },
    Returned {
        id: BookId
    }
}

impl Display for PersonEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PersonManipulation :: {:?}", self)
    }
}

impl Applier<PersonEvent> for Person {
    fn apply(&mut self, event: PersonEvent) {
        match event {
            PersonEvent::Created { id, name } => {
                self.substitute(|person| {
                    *person.id = id;
                    *person.name = name;
                })
            },
            PersonEvent::Renamed { name } => {
                self.substitute(|person| {
                    *person.name = name;
                })
            },
            PersonEvent::Rented { .. } => {
                self.substitute(|person| {
                    person.rental.apply(event);
                })
            }
            PersonEvent::Returned { .. } => {
                self.substitute(|person| {
                    person.rental.apply(event)
                })
            }
        }
    }
}


#[cfg(test)]
mod test {
    use error_stack::{Report, ResultExt};
    use crate::entities::{PersonId, PersonName};
    use crate::error::test::AnyKernelError;
    use crate::event::PersonEvent;
    
    #[test]
    fn serialize() -> Result<(), Report<AnyKernelError>> {
        let ev = PersonEvent::Created {
            id: PersonId::default(),
            name: PersonName::new("test_man")
        };
        let se = serde_json::to_string(&ev)
            .change_context_lazy(|| AnyKernelError)?;
        
        dbg!(se);
        Ok(())
    }
}