use kernel::interfaces::repository::{DependOnPersonRepository, PersonRepository};
use kernel::prelude::entities::{Person, PersonId, PersonName};
use crate::error::ApplicationError;
use crate::transfer::person::{CreatePersonDto, DeletePersonDto, PersonDto, UpdatePersonDto};

#[async_trait::async_trait]
pub trait CreatePersonService: 'static + Sync + Send
    + DependOnPersonRepository
{
    async fn create(&self, create: CreatePersonDto) -> Result<PersonDto, ApplicationError> {
        let CreatePersonDto { name } = create;

        let id = PersonId::default();
        let name = PersonName::new(name);
        let person = Person::new(id, name);

        self.person_repository().create(&person).await?;

        Ok(person.into())
    }
}

pub trait DependOnCreatePersonService: 'static + Sync + Send {
    type CreatePersonService: CreatePersonService;
    fn create_person_service(&self) -> &Self::CreatePersonService;
}


#[async_trait::async_trait]
pub trait UpdatePersonService: 'static + Sync + Send
    + DependOnPersonRepository
{
    // noinspection DuplicatedCode
    async fn update(&self, update: UpdatePersonDto) -> Result<PersonDto, ApplicationError> {
        let UpdatePersonDto { id, name } = update;

        let id = PersonId::new(id);
        
        let Some(person) = self.person_repository().find_by_id(&id).await? else {
            return Err(ApplicationError::NotFound {
                entity: "Person",
                method: "find_by_id",
                value: id.into(),
            })
        };
        
        let mut person = person.into_destruct();
        
        person.name = PersonName::new(name);

        let person = person.freeze();
        
        self.person_repository().update(&person).await?;
        
        Ok(person.into())
    }
}

pub trait DependOnUpdatePersonService: 'static + Sync + Send {
    type UpdatePersonService: UpdatePersonService;
    fn update_person_service(&self) -> &Self::UpdatePersonService;
}


#[async_trait::async_trait]
pub trait DeletePersonService: 'static + Sync + Send
    + DependOnPersonRepository
{
    // noinspection DuplicatedCode
    async fn delete(&self, delete: DeletePersonDto) -> Result<PersonDto, ApplicationError> {
        let DeletePersonDto { id } = delete;
        
        let id = PersonId::new(id);

        let Some(person) = self.person_repository().find_by_id(&id).await? else {
            return Err(ApplicationError::NotFound {
                entity: "Person",
                method: "find_by_id",
                value: id.into(),
            })
        };
        
        self.person_repository().delete(person.id()).await?;
        
        Ok(person.into())
    }
}

pub trait DependOnDeletePersonService: 'static + Sync + Send {
    type DeletePersonService: DeletePersonService;
    fn delete_person_service(&self) -> &Self::DeletePersonService;
}