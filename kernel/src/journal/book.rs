use std::future::Future;
use error_stack::Report;
use crate::entities::{Book, BookId};
use crate::error::KernelError;
use crate::event::BookEvent;
use crate::io::{AcquireTransaction, DependOnAcquireTransaction, Transaction};
use crate::journal::Envelope;

pub trait BookEventJournal: 'static + Sync + Send {
    type Transaction: Transaction;
    fn create(&self, event: &BookEvent, con: &mut Self::Transaction) -> impl Future<Output = Result<(), Report<KernelError>>> + Send;
    fn append(&self, id: &BookId, event: &BookEvent, con: &mut Self::Transaction) -> impl Future<Output = Result<(), Report<KernelError>>> + Send;
    fn replay(&self, id: &BookId, con: &mut Self::Transaction) -> impl Future<Output = Result<Envelope<Book>, Report<KernelError>>> + Send;
    fn resume(&self, envelope: &mut Envelope<Book>, con: &mut Self::Transaction) -> impl Future<Output = Result<(), Report<KernelError>>> + Send;
}

pub trait DependOnBookEventJournal: 'static + Sync + Send + DependOnAcquireTransaction {
    type BookEventJournal: BookEventJournal<Transaction=<Self::AcquireTransaction as AcquireTransaction>::Transaction>;
    fn book_event_journal(&self) -> Self::BookEventJournal;
}
