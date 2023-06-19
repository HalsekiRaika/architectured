use crate::entities::{Person, PersonId};
use crate::error::KernelError;


#[async_trait::async_trait]
pub trait PersonRepository: Sync + Send + 'static {
    async fn create(&self, create: &Person) -> Result<(), KernelError>;
    async fn update(&self, update: &Person) -> Result<(), KernelError>;
    async fn delete(&self, delete: &PersonId) -> Result<(), KernelError>;

    async fn find_by_id(&self, id: &PersonId) -> Result<Option<Person>, KernelError>;
}


pub trait DependOnPersonRepository: Sync + Send + 'static {
    type PersonRepository: PersonRepository;
    fn person_repository(&self) -> &Self::PersonRepository;
}