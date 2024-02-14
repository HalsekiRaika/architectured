use std::ops::Deref;
use std::sync::Arc;
use application::services::DependOnPersonCommandExecutionService;
use driver::database::PersonDataBase;
use driver::journal::PersonEventJournal;
use kernel::interfaces::repository::DependOnPersonRepository;
use kernel::interfaces::journal::DependOnPersonManipulationEventJournal;
use crate::error::ServerError;

pub struct AppModule(Arc<Handler>);

impl AppModule {
    pub async fn new() -> Result<Self, ServerError> {
        Ok(Self(Arc::new(Handler::init().await?)))
    }
}

impl Clone for AppModule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Deref for AppModule {
    type Target = Handler;
    fn deref(&self) -> &Self::Target {
        Deref::deref(&self.0)
    }
}

pub struct Handler {
    person_db: PersonDataBase,
    person_journal: PersonEventJournal
}

impl Handler {
    async fn init() -> Result<Self, ServerError> {
        let journal_pool = driver::setup_journal_db().await?;
        let redis_pool = driver::setup_redis()?;


        Ok(Self {
            person_db: PersonDataBase::new(redis_pool),
            person_journal: PersonEventJournal::new(journal_pool),
        })
    }
}

impl DependOnPersonRepository for Handler {
    type PersonRepository = PersonDataBase;
    fn person_repository(&self) -> &Self::PersonRepository {
        &self.person_db
    }
}

impl DependOnPersonManipulationEventJournal for Handler {
    type PersonManipulationEventJournal = PersonEventJournal;
    fn person_manipulation_event_journal(&self) -> &Self::PersonManipulationEventJournal {
        &self.person_journal
    }
}

impl DependOnPersonCommandExecutionService for Handler {
    type PersonCommandExecutionService = Self;
    fn person_command_execution_service(&self) -> &Self::PersonCommandExecutionService {
        self
    }
}