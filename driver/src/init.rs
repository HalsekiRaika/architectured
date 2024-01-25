use deadpool_redis::{Config, Pool as RedisPool};
use error_stack::{Report, ResultExt};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::error::DriverError;

pub fn setup_redis() -> Result<RedisPool, Report<DriverError>> {
    Config::from_url(dotenvy::var("REDIS_CONNECTION").expect("require set `REDIS_CONNECTION`"))
        .create_pool(deadpool_redis::Runtime::Tokio1.into())
        .change_context_lazy(|| DriverError::Initialize)
}

pub async fn setup_query_db() -> Result<Pool<Postgres>, DriverError> {
    const KEY: &str = "QUERY_DATABASE_URL";

    let url = dotenvy::var(KEY)
        .unwrap_or_else(|_| panic!("`{KEY}` does not set. This value required!"));

    let pool = PgPoolOptions::new()
        .connect(&url)
        .await?;

    sqlx::migrate!("../migrations/query")
        .run(&pool)
        .await?;

    Ok(pool)
}

pub async fn setup_journal_db() -> Result<Pool<Postgres>, DriverError> {
    const KEY: &str = "JOURNAL_DATABASE_URL";

    let url = dotenvy::var(KEY)
        .unwrap_or_else(|_| panic!("`{KEY}` does not set. This value required!"));

    let pool = PgPoolOptions::new()
        .connect(&url)
        .await?;

    sqlx::migrate!("../migrations/journal")
        .run(&pool)
        .await?;

    Ok(pool)
}


#[cfg(test)]
mod test {
    use error_stack::Report;
    use uuid::Uuid;
    use kernel::prelude::events::PersonManipulationEvent;
    use crate::{error::{DriverError, test::AnyDriverError}, setup_journal_db};
    
    #[tokio::test]
    async fn cbor_insert() -> Result<(), Report<AnyDriverError>> {
        let pool = setup_journal_db().await?;
        
        let mut transaction = pool.begin()
            .await
            .map_err(DriverError::from)?;
       
        let ev = PersonManipulationEvent::Created { id: Default::default(), name: Default::default() };
        
        let mut cbor = Vec::new();
        ciborium::ser::into_writer(&ev, &mut cbor)
            .map_err(|_| DriverError::Other)?;
        
        let stream_id = Uuid::new_v4();
        
        // language=SQL
        sqlx::query(r#"
          INSERT INTO streams(id, version) VALUES ($1, $2)
        "#)
            .bind(stream_id)
            .bind(1)
            .execute(&mut *transaction)
            .await
            .map_err(DriverError::from)?;
        
        // language=SQL
        let cbor = sqlx::query_scalar::<_, Vec<u8>>(r#"
          INSERT INTO events(journal, version, event) VALUES ($1, $2, $3) RETURNING (event)
        "#)
            .bind(stream_id)
            .bind(1)
            .bind(cbor.as_slice())
            .fetch_one(&mut *transaction)
            .await
            .map_err(DriverError::from)?;
        
        // language=SQL
        // let cbor = sqlx::query_scalar::<_, Vec<u8>>(r#"
        //   SELECT (event) FROM events WHERE id = 1::SERIAL
        // "#)
        //     .fetch_one(&mut *transaction)
        //     .await
        //     .map_err(DriverError::from)?;
       
        println!("{:?}", &cbor);
        
        let mut vec = Vec::new();
        let ev: PersonManipulationEvent = ciborium::de::from_reader_with_buffer(cbor.as_slice(), &mut vec)
            .map_err(|e| {
                println!("{:?}", e);
                DriverError::Other
            })?;
        
        transaction.rollback().await
            .map_err(DriverError::from)?;
        
        Ok(())
    }
}