use application::services::{DependOnCreatePersonService, DependOnDeletePersonService, DependOnUpdatePersonService};
use kernel::interfaces::repository::DependOnPersonRepository;
use driver::database::PersonDataBase;
use crate::error::ServerError;

pub struct Handler {
    person_db: PersonDataBase
}

impl Handler {
    pub async fn init() -> Result<Self, ServerError> {
        let redis_pool = driver::setup()?;

        Ok(Self {
            person_db: PersonDataBase::new(redis_pool),
        })
    }
}

impl DependOnPersonRepository for Handler {
    type PersonRepository = PersonDataBase;
    fn person_repository(&self) -> &Self::PersonRepository {
        &self.person_db
    }
}

impl DependOnCreatePersonService for Handler {
    type CreatePersonService = Self;
    fn create_person_service(&self) -> &Self::CreatePersonService {
        self
    }
}

impl DependOnUpdatePersonService for Handler {
    type UpdatePersonService = Self;
    fn update_person_service(&self) -> &Self::UpdatePersonService {
        self
    }
}

impl DependOnDeletePersonService for Handler {
    type DeletePersonService = Self;
    fn delete_person_service(&self) -> &Self::DeletePersonService {
        self
    }
}