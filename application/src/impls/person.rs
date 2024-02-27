use kernel::interfaces::io::{DependOnAcquireTransaction};
use kernel::interfaces::journal::{DependOnBookEventJournal, DependOnPersonManipulationEventJournal};
use crate::service::PersonCommandExecutionService;

impl<T> PersonCommandExecutionService for T
    where T: DependOnPersonManipulationEventJournal
           + DependOnBookEventJournal
           + DependOnAcquireTransaction {}