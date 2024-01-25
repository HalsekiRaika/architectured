use error_stack::Report;
use crate::entities::{Person, PersonId};
use crate::error::KernelError;


#[async_trait::async_trait]
pub trait PersonRepository: Sync + Send + 'static {
    async fn create(&self, create: &Person) -> Result<(), Report<KernelError>>;
    async fn update(&self, update: &Person) -> Result<(), Report<KernelError>>;
    async fn delete(&self, delete: &PersonId) -> Result<(), Report<KernelError>>;

    async fn find_by_id(&self, id: &PersonId) -> Result<Option<Person>,Report<KernelError>>;
}


pub trait DependOnPersonRepository: Sync + Send + 'static {
    type PersonRepository: PersonRepository;
    fn person_repository(&self) -> &Self::PersonRepository;
}