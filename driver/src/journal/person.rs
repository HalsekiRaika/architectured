use error_stack::Report;
use sqlx::{PgConnection, Pool, Postgres};
use kernel::error::KernelError;
use kernel::interfaces::journal::PersonManipulationEventJournal;
use kernel::prelude::events::PersonManipulationEvent;
use crate::error::DriverError;
use crate::error::internal::InternalError;

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
    pub(crate) async fn create(
        event: &PersonManipulationEvent,
        con: &mut PgConnection
    ) -> Result<(), DriverError> {
        let PersonManipulationEvent::Created { id, .. } = event else {
            return Err(InternalError::Constraint("this event containing data should be for stream.").into())
        };
        
        // language=SQL
        sqlx::query(r#"
            INSERT INTO streams(id, version) VALUES ($1, $2)
        "#)
            .bind(id.as_ref())
            .bind(1)
            .execute(&mut *con)
            .await?;
        
        // language=SQL
        sqlx::query(r#"
            INSERT INTO events(stream, version, event) VALUES ($1, $2, $3)
        "#)
            .bind(id.as_ref())
            .bind(1)
            .bind(serde_json::to_value(event)?)
            .execute(&mut *con)
            .await?;
        Ok(())
    }
    
    pub(crate) async fn append(
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


#[cfg(test)]
mod test {
    use sqlx::{Pool, Postgres};
    use crate::error::DriverError;
    use crate::setup_journal_db;
    
    async fn create_pool() -> Result<Pool<Postgres>, DriverError> {
        let pool = setup_journal_db().await?;
        Ok(pool)
    }
    
}