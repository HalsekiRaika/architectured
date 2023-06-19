mod id;
mod name;

pub use self::{
    id::*,
    name::*,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, destructure::Destructure)]
pub struct Person {
    id: PersonId,
    name: PersonName
}

impl Person {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id: PersonId::new(id),
            name: PersonName::new(name),
        }
    }
}

impl Person {
    pub fn id(&self) -> &PersonId {
        &self.id
    }
    
    pub fn name(&self) -> &PersonName {
        &self.name
    }
}