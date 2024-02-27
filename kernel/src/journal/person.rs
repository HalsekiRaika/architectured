use std::future::Future;
use error_stack::Report;
use crate::entities::{Person, PersonId};
use crate::error::KernelError;
use crate::event::PersonEvent;
use crate::io::{AcquireTransaction, DependOnAcquireTransaction, Transaction};
use crate::journal::Envelope;

pub trait PersonManipulationEventJournal: 'static + Sync + Send {
    type Transaction: Transaction;
    fn create(&self, event: &PersonEvent, con: &mut Self::Transaction) -> impl Future<Output = Result<(), Report<KernelError>>> + Send;
    fn append(&self, id: &PersonId, event: &PersonEvent, con: &mut Self::Transaction) -> impl Future<Output = Result<(), Report<KernelError>>> + Send;
    fn replay(&self, id: &PersonId, con: &mut Self::Transaction) -> impl Future<Output = Result<Envelope<Person>, Report<KernelError>>> + Send;
    fn resume(&self, envelope: &mut Envelope<Person>, con: &mut Self::Transaction) -> impl Future<Output = Result<(), Report<KernelError>>> + Send;
}

pub trait DependOnPersonManipulationEventJournal: 'static + Sync + Send + DependOnAcquireTransaction {
    type PersonManipulationEventJournal: PersonManipulationEventJournal<Transaction=<Self::AcquireTransaction as AcquireTransaction>::Transaction>;
    fn person_manipulation_event_journal(&self) -> Self::PersonManipulationEventJournal;
}