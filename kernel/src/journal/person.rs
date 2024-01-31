use std::future::Future;
use crate::entities::{Person, PersonId};
use crate::event::PersonManipulationEvent;
use crate::journal::Envelope;

#[orbital::export_service]
pub trait PersonManipulationEventJournal: 'static + Sync + Send {
    type Error;
    fn create(&self, event: &PersonManipulationEvent) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn append(&self, id: &PersonId, event: &PersonManipulationEvent) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn replay(&self, id: &PersonId) -> impl Future<Output = Result<Envelope<Person>, Self::Error>> + Send;
    fn resume(&self, envelope: &mut Envelope<Person>) -> impl Future<Output = Result<(), Self::Error>> + Send;
}