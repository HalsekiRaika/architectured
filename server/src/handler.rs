use std::ops::Deref;
use std::sync::Arc;
use application::services::{DependOnBookCommandExecutionService, DependOnPersonCommandExecutionService};
use driver::database::PgPool;
use driver::journal::{BookEventRecord, PersonEventRecord};
use kernel::interfaces::io::DependOnAcquireTransaction;
use kernel::interfaces::journal::{DependOnBookEventJournal, DependOnPersonManipulationEventJournal};
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
    pgpool: PgPool
}

impl Handler {
    async fn init() -> Result<Self, ServerError> {
        let pgpool = driver::setup_journal_db().await?;

        Ok(Self {
            pgpool
        })
    }
}


impl DependOnAcquireTransaction for Handler {
    type AcquireTransaction = PgPool;
    
    fn acquire_transaction(&self) -> &Self::AcquireTransaction {
        &self.pgpool
    }
}

impl DependOnPersonManipulationEventJournal for Handler {
    type PersonManipulationEventJournal = PersonEventRecord;
    fn person_manipulation_event_journal(&self) -> PersonEventRecord {
        PersonEventRecord
    }
}

impl DependOnBookEventJournal for Handler {
    type BookEventJournal = BookEventRecord;
    fn book_event_journal(&self) -> BookEventRecord {
        BookEventRecord
    }

}

impl DependOnPersonCommandExecutionService for Handler {
    type PersonCommandExecutionService = Self;
    fn person_command_execution_service(&self) -> &Self::PersonCommandExecutionService {
        self
    }
}

impl DependOnBookCommandExecutionService for Handler {
    type BookCommandExecutionService = Self;
    fn book_command_execution_service(&self) -> &Self::BookCommandExecutionService {
        self
    }
}