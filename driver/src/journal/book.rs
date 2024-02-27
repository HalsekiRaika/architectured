use error_stack::Report;
use sqlx::PgConnection;
use kernel::error::KernelError;
use kernel::interfaces::journal::{BookEventJournal, Envelope};
use kernel::prelude::entities::{Book, BookId};
use kernel::prelude::events::{Applier, BookEvent};
use crate::database::PgTransaction;
use crate::error::DriverError;
use crate::error::internal::InternalError;

#[derive(Default)]
pub struct BookEventRecord;

impl BookEventJournal for BookEventRecord {
    type Transaction = PgTransaction;
    
    async fn create(&self, event: &BookEvent, con: &mut Self::Transaction) -> Result<(), Report<KernelError>> {
        InternalBookEventJournal::create(event, con).await?;
        Ok(())
    }
    
    async fn append(&self, id: &BookId, event: &BookEvent, con: &mut Self::Transaction) -> Result<(), Report<KernelError>> {
        InternalBookEventJournal::append(id, event, con).await?;
        Ok(())
    }
    
    async fn replay(&self, id: &BookId, con: &mut Self::Transaction) -> Result<Envelope<Book>, Report<KernelError>> {
        let replay = InternalBookEventJournal::replay(id, con).await?;
        Ok(replay)
    }
    
    async fn resume(&self, envelope: &mut Envelope<Book>, con: &mut Self::Transaction) -> Result<(), Report<KernelError>> {
        InternalBookEventJournal::resume(envelope, con).await?;
        Ok(())
    }
}


pub(crate) struct InternalBookEventJournal;

// noinspection DuplicatedCode
impl InternalBookEventJournal {
    pub(crate) async fn create(
        event: &BookEvent,
        con: &mut PgConnection
    ) -> Result<(), DriverError> {
        let BookEvent::Arrival { id, .. } = event else {
            return Err(InternalError::Constraint("this event containing data should be for create stream.").into())
        };
        
        // language=SQL
        sqlx::query(r#"
            INSERT INTO streams(id) VALUES ($1)
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
        stream: &BookId,
        event: &BookEvent,
        con: &mut PgConnection
    ) -> Result<(), DriverError> {
        // language=SQL
        let current = sqlx::query_scalar::<_, i64>(r#"
            SELECT COUNT(event) from events WHERE stream = $1
        "#)
            .bind(stream.as_ref())
            .fetch_one(&mut *con)
            .await?;
        
        // language=SQL
        sqlx::query(r#"
            INSERT INTO events(stream, event, version) VALUES ($1, $2, $3)
        "#)
            .bind(stream.as_ref())
            .bind(serde_json::to_value(event)?)
            .bind(current + 1)
            .execute(&mut *con)
            .await?;
        Ok(())
    }
    
    pub(crate) async fn replay(
        stream: &BookId,
        con: &mut PgConnection
    ) -> Result<Envelope<Book>, DriverError> {
        // language=SQL
        let events = sqlx::query_scalar::<_, serde_json::Value>(r#"
            SELECT (event) FROM events WHERE stream = $1
        "#)
            .bind(stream.as_ref())
            .fetch_all(&mut *con)
            .await?;
        
        let events = events.into_iter()
            .map(serde_json::from_value)
            .collect::<Result<Vec<BookEvent>, _>>()?;
        
        let mut book = Book::default();
        
        let version = events.iter().fold(0, |mut c, _| { c += 1; c });
        
        events.into_iter()
            .for_each(|event| {
                book.apply(event)
            });
        
        let envelope = Envelope::new(version, book);
        
        Ok(envelope)
    }
    
    pub(crate) async fn resume(
        book: &mut Envelope<Book>,
        con: &mut PgConnection
    ) -> Result<(), DriverError> {
        // language=SQL
        let resume = sqlx::query_scalar::<_, serde_json::Value>(r#"
            SELECT (event) FROM events ev WHERE stream = $1 AND $2 <= ev.version
        "#)
            .bind(book.id().as_ref())
            .bind(book.version())
            .fetch_all(&mut *con)
            .await?;
        
        let version = resume.iter().fold(0, |mut c, _| { c += 1; c });
        
        resume.into_iter()
            .map(serde_json::from_value)
            .collect::<Result<Vec<BookEvent>, _>>()?
            .into_iter()
            .for_each(|ev| {
                book.apply(ev)
            });
        
        book.version = version;
        
        Ok(())
    }
}
