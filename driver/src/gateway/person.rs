use futures::StreamExt;
use rdkafka::Message;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use kernel::prelude::entities::PersonId;
use kernel::prelude::events::PersonManipulationEvent;
use crate::error::DriverError;

const TOPIC: &str = "person";

pub(crate) struct InternalPersonCommandProducer;

impl InternalPersonCommandProducer {
    async fn produce(
        id: &PersonId,
        event: &PersonManipulationEvent,
        prod: &FutureProducer
    ) -> Result<(), DriverError> {
        let payload = serde_json::to_string(event)?;
        let record = FutureRecord::to(TOPIC)
            .key(id.as_ref().as_bytes())
            .payload(&payload);

        prod.send_result(record)
            .map_err(|e| {
                e.0
            })?
            .await
            .map_err(|c| DriverError::Other)?
            .map_err(|e| {
                println!("{:?}", e.1);
                e.0
            })?;

        Ok(())
    }
}

pub(crate) struct InternalPersonCommandConsumer;

impl InternalPersonCommandConsumer {
    async fn consume(cons: &StreamConsumer) -> Result<(), DriverError> {
        cons.subscribe(&[TOPIC]).map_err(|_| DriverError::Other)?;

        while let Some(payload) = cons.stream().next().await {
            let msg = payload?;
            let viw = msg.payload_view()
                .transpose()
                .map_err(|_| DriverError::Other)?;
            let viw = viw.map(serde_json::from_str::<PersonManipulationEvent>);

            let Some(msg) = viw.transpose()? else {
                continue;
            };

            println!("msg: {}", msg);
        }

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use error_stack::Report;
    use rdkafka::ClientConfig;
    use kernel::prelude::commands::{PersonManipulationCommand, Publication};
    use super::*;

    fn create_prod_cons() -> Result<(FutureProducer, StreamConsumer), DriverError> {
        let mut client = ClientConfig::new();
        client.set("bootstrap.servers", "localhost:29092");
        client.set("message.timeout.ms", "5000");

        let producer: FutureProducer = client.create()?;
        let consumer: StreamConsumer = client.create()?;

        Ok((producer, consumer))
    }

    #[tokio::test]
    async fn pub_sub() -> Result<(), Report<DriverError>> {
        let (prod, cons) = create_prod_cons()?;

        let cmd = PersonManipulationCommand::Create { name: "kafka_test".to_string() };
        let evt = cmd.publish().map_err(|_| DriverError::Other)?;

        match evt {
            PersonManipulationEvent::Created { ref id, .. } => {
                InternalPersonCommandProducer::produce(id, &evt, &prod).await?;
            }
            PersonManipulationEvent::Renamed { .. } => {

            }
        }

        tokio::spawn(async move {
            InternalPersonCommandConsumer::consume(&cons).await
        }).await.map_err(|e| DriverError::Other)??;

        Ok(())
    }
}