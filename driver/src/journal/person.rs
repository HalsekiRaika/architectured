use error_stack::Report;
use sqlx::{PgConnection, Pool, Postgres};
use kernel::error::KernelError;
use kernel::interfaces::journal::PersonManipulationEventJournal;
use kernel::prelude::events::PersonManipulationEvent;
use crate::error::DriverError;

pub struct PersonEventJournal {
    pool: Pool<Postgres>
}

impl PersonEventJournal {
    pub fn new(pool: Pool<Postgres>) -> PersonEventJournal {
        Self { pool }
    }
}


impl PersonManipulationEventJournal for PersonEventJournal {
    type Error = Report<KernelError>;

    async fn save(&self, event: &PersonManipulationEvent) -> Result<(), Self::Error> {
        todo!()
    }
}

pub(crate) struct InternalPersonEventJournal;

impl InternalPersonEventJournal {
    pub(crate) async fn save(
        event: &PersonManipulationEvent,
        con: &mut PgConnection
    ) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(r#"
        
        "#)
            .execute(&mut *con)
            .await?;
        Ok(())
    }
}