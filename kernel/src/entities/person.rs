mod id;
mod name;
mod rental;

pub use self::{
    id::*,
    name::*,
    rental::*,
};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, destructure::Destructure, destructure::Mutation)]
pub struct Person {
    id: PersonId,
    name: PersonName,
    rental: Rental,
}

impl Person {
    pub fn new(id: PersonId, name: PersonName, rental: Rental) -> Self {
        Self { id, name, rental, }
    }
}

impl Person {
    pub fn id(&self) -> &PersonId {
        &self.id
    }
    
    pub fn name(&self) -> &PersonName {
        &self.name
    }
    
    pub fn rental(&self) -> &Rental {
        &self.rental
    }
}