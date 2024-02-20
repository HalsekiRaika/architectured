use kernel::interfaces::io::{DependOnAcquireTransaction};
use kernel::interfaces::journal::DependOnPersonManipulationEventJournal;
use crate::service::PersonCommandExecutionService;

impl<T> PersonCommandExecutionService for T
    where T: DependOnPersonManipulationEventJournal
           + DependOnAcquireTransaction {}