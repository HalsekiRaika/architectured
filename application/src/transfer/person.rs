use kernel::prelude::entities::{DestructPerson, Person};

#[derive(Debug)]
pub struct PersonDto {
    pub id: String,
    pub name: String
}

impl From<Person> for PersonDto {
    fn from(value: Person) -> Self {
        let DestructPerson {
            id,
            name
        } = value.into_destruct();
        Self {
            id: id.into(),
            name: name.into()
        }
    }
}

pub struct CreatePersonDto {
    pub name: String
}

pub struct UpdatePersonDto {
    pub id: String,
    pub name: String
}

pub struct DeletePersonDto {
    pub id: String
}