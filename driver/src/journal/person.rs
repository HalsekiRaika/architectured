use error_stack::Report;
use sqlx::PgConnection;
use kernel::error::KernelError;
use kernel::interfaces::journal::{Envelope, PersonManipulationEventJournal};
use kernel::prelude::entities::{Person, PersonId};
use kernel::prelude::events::{Applier, PersonManipulationEvent};
use crate::database::PgTransaction;
use crate::error::DriverError;
use crate::error::internal::InternalError;

#[derive(Default)]
pub struct PersonEventJournal;

impl PersonManipulationEventJournal for PersonEventJournal {
    type Transaction = PgTransaction;
    
    async fn create(&self, event: &PersonManipulationEvent, con: &mut Self::Transaction) -> Result<(), Report<KernelError>> {
        InternalPersonEventJournal::create(event, con).await?;
        Ok(())
    }

    async fn append(&self, id: &PersonId, event: &PersonManipulationEvent, con: &mut Self::Transaction) -> Result<(), Report<KernelError>> {
        InternalPersonEventJournal::append(id, event, con).await?;
        Ok(())
    }

    async fn replay(&self, id: &PersonId, con: &mut Self::Transaction) -> Result<Envelope<Person>, Report<KernelError>> {
        let replay = InternalPersonEventJournal::replay(id, con).await?;
        Ok(replay)
    }

    async fn resume(&self, envelope: &mut Envelope<Person>, con: &mut Self::Transaction) -> Result<(), Report<KernelError>> {
        InternalPersonEventJournal::resume(envelope, con).await?;
        Ok(())
    }
}


pub(crate) struct InternalPersonEventJournal;

impl InternalPersonEventJournal {
    pub(crate) async fn create(
        event: &PersonManipulationEvent,
        con: &mut PgConnection
    ) -> Result<(), DriverError> {
        let PersonManipulationEvent::Created { id, .. } = event else {
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
        stream: &PersonId,
        event: &PersonManipulationEvent,
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
        stream: &PersonId,
        con: &mut PgConnection
    ) -> Result<Envelope<Person>, DriverError> {
        // language=SQL
        let events = sqlx::query_scalar::<_, serde_json::Value>(r#"
            SELECT (event) FROM events WHERE stream = $1
        "#)
            .bind(stream.as_ref())
            .fetch_all(&mut *con)
            .await?;

        let events = events.into_iter()
            .map(serde_json::from_value)
            .collect::<Result<Vec<PersonManipulationEvent>, _>>()?;

        let mut person = Person::default();

        let version = events.iter().fold(0, |mut c, _| { c += 1; c });

        events.into_iter()
            .for_each(|event| {
                person.apply(event)
            });

        let envelope = Envelope::new(version, person);

        Ok(envelope)
    }

    pub(crate) async fn resume(
        person: &mut Envelope<Person>,
        con: &mut PgConnection
    ) -> Result<(), DriverError> {
        // language=SQL
        let resume = sqlx::query_scalar::<_, serde_json::Value>(r#"
            SELECT (event) FROM events ev WHERE stream = $1 AND $2 <= ev.version
        "#)
            .bind(person.id().as_ref())
            .bind(person.version())
            .fetch_all(&mut *con)
            .await?;

        let version = resume.iter().fold(0, |mut c, _| { c += 1; c });

        resume.into_iter()
            .map(serde_json::from_value)
            .collect::<Result<Vec<PersonManipulationEvent>, _>>()?
            .into_iter()
            .for_each(|ev| {
                person.apply(ev)
            });

        person.version = version;

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use error_stack::Report;
    use futures::StreamExt;
    use tokio::time::Instant;
    use kernel::prelude::entities::{PersonId, PersonName};
    use kernel::prelude::events::PersonManipulationEvent;
    use crate::database::PgPool;
    use crate::error::DriverError;
    use crate::journal::person::InternalPersonEventJournal;
    use crate::setup_journal_db;
    
    async fn create_pool() -> Result<PgPool, DriverError> {
        let pool = setup_journal_db().await?;
        Ok(pool)
    }

    #[tokio::test]
    async fn append_event() -> Result<(), Report<DriverError>> {
        let id = PersonId::default();
        let ev_1 = PersonManipulationEvent::Created { id, name: PersonName::new("new account 1") };
        let ev_2 = PersonManipulationEvent::Renamed { name: PersonName::new("test man") };

        let mut transaction = create_pool().await?
            .begin().await
            .map_err(DriverError::from)?;

        InternalPersonEventJournal::create(&ev_1, &mut transaction).await?;
        InternalPersonEventJournal::append(&id, &ev_2, &mut transaction).await?;

        transaction.rollback().await
            .map_err(DriverError::from)?;

        Ok(())
    }

    // noinspection DuplicatedCode
    #[tokio::test]
    async fn append_parallel() -> Result<(), DriverError> {
        let id_a = PersonId::default();
        let id_b = PersonId::default();

        println!("id_a: {:?}", id_a);
        println!("id_b: {:?}", id_b);

        let pool = create_pool().await?;

        let a: Vec<PersonManipulationEvent> = futures::stream::iter(1..=2500)
            .map(|i| async move {
                PersonManipulationEvent::Renamed { name: PersonName::new(format!("test man type.{i}")) }
            })
            .buffered(1)
            .collect::<_>().await;

        let b = a.clone();

        let pool_a = pool.clone();
        let mut transaction_a = pool_a.begin().await.map_err(DriverError::from)?;

        let ev_1 = PersonManipulationEvent::Created { id: id_a, name: PersonName::new("new account 1") };
        InternalPersonEventJournal::create(&ev_1, &mut transaction_a).await?;

        let task_a = futures::stream::iter(a)
            .fold(&mut transaction_a, |con, ev| async move {
                InternalPersonEventJournal::append(&id_a, &ev, con).await.unwrap();
                con
            });

        let pool_b = pool.clone();
        let mut transaction_b = pool_b.begin().await.map_err(DriverError::from)?;

        let ev_2 = PersonManipulationEvent::Created { id: id_b, name: PersonName::new("new account 1") };
        InternalPersonEventJournal::create(&ev_2, &mut transaction_b).await?;

        let task_b = futures::stream::iter(b)
            .fold(&mut transaction_b, |con, ev| async move {
                InternalPersonEventJournal::append(&id_b, &ev, con).await.unwrap();
                con
            });

        futures::join!(task_a, task_b);


        let person_a = InternalPersonEventJournal::replay(&id_a, &mut transaction_a).await?;
        let person_b = InternalPersonEventJournal::replay(&id_b, &mut transaction_b).await?;

        println!("{:#?}", person_a);
        println!("{:#?}", person_b);

        transaction_a.rollback().await.map_err(DriverError::from)?;
        transaction_b.rollback().await.map_err(DriverError::from)?;

        Ok(())
    }

    #[tokio::test]
    async fn replay_event() -> Result<(), Report<DriverError>> {
        let id = PersonId::default();
        let ev_1 = PersonManipulationEvent::Created { id, name: PersonName::new("new account 1") };
        let ev_2 = PersonManipulationEvent::Renamed { name: PersonName::new("test man") };

        let mut transaction = create_pool().await?
            .begin().await
            .map_err(DriverError::from)?;

        InternalPersonEventJournal::create(&ev_1, &mut transaction).await?;
        InternalPersonEventJournal::append(&id, &ev_2, &mut transaction).await?;

        futures::stream::iter(1..10000)
            .map(|i| async move {
                PersonManipulationEvent::Renamed { name: PersonName::new(format!("test man type.{i}")) }
            })
            .buffered(1)
            .fold(&mut transaction, |con, ev| async move {
                InternalPersonEventJournal::append(&id, &ev, con).await.unwrap();
                con
            }).await;

        let now = Instant::now();

        let person = InternalPersonEventJournal::replay(&id, &mut transaction).await?;

        let elapsed = now.elapsed();

        assert_eq!(person.id(), &id);

        println!("replay {}sec", elapsed.as_secs_f64());
        println!("{:#?}", person);

        transaction.rollback().await
            .map_err(DriverError::from)?;

        Ok(())
    }

    #[tokio::test]
    async fn resume_event() -> Result<(), Report<DriverError>> {
        let id = PersonId::default();
        let ev_1 = PersonManipulationEvent::Created { id, name: PersonName::new("new account 1") };

        let mut transaction = create_pool().await?
            .begin().await
            .map_err(DriverError::from)?;

        InternalPersonEventJournal::create(&ev_1, &mut transaction).await?;

        futures::stream::iter(1..1000)
            .map(|i| async move {
                PersonManipulationEvent::Renamed { name: PersonName::new(format!("test man type.{i}")) }
            })
            .buffered(1)
            .fold(&mut transaction, |con, ev| async move {
                InternalPersonEventJournal::append(&id, &ev, con).await.unwrap();
                con
            }).await;

        let mut person = InternalPersonEventJournal::replay(&id, &mut transaction).await?;

        futures::stream::iter(1000..=2000)
            .map(|i| async move {
                PersonManipulationEvent::Renamed { name: PersonName::new(format!("test man type.{i}")) }
            })
            .buffered(1)
            .fold(&mut transaction, |con, ev| async move {
                InternalPersonEventJournal::append(&id, &ev, con).await.unwrap();
                con
            }).await;

        let now = Instant::now();

        InternalPersonEventJournal::resume(&mut person, &mut transaction).await?;

        let elapsed = now.elapsed();

        println!("resume {}sec", elapsed.as_secs_f64());
        println!("{:#?}", person);

        Ok(())
    }
}