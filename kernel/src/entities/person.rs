mod id;
mod name;

pub use self::{
    id::*,
    name::*,
};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, destructure::Destructure, destructure::Mutation)]
pub struct Person {
    id: PersonId,
    name: PersonName
}

impl Person {
    pub fn new(id: PersonId, name: PersonName) -> Self {
        Self { id, name, }
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