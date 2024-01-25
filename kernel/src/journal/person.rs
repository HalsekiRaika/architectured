use std::future::Future;
use crate::event::PersonManipulationEvent;

#[orbital::export_service]
pub trait PersonManipulationEventJournal: 'static + Sync + Send {
    type Error;
    fn save(&self, event: &PersonManipulationEvent) -> impl Future<Output = Result<(), Self::Error>> + Send;
}