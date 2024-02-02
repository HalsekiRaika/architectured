use std::future::Future;
use error_stack::Report;
use crate::entities::{Person, PersonId};
use crate::error::KernelError;
use crate::event::PersonManipulationEvent;
use crate::journal::Envelope;

#[orbital::export_service]
pub trait PersonManipulationEventJournal: 'static + Sync + Send {
    fn create(&self, event: &PersonManipulationEvent) -> impl Future<Output = Result<(), Report<KernelError>>> + Send;
    fn append(&self, id: &PersonId, event: &PersonManipulationEvent) -> impl Future<Output = Result<(), Report<KernelError>>> + Send;
    fn replay(&self, id: &PersonId) -> impl Future<Output = Result<Envelope<Person>, Report<KernelError>>> + Send;
    fn resume(&self, envelope: &mut Envelope<Person>) -> impl Future<Output = Result<(), Report<KernelError>>> + Send;
}