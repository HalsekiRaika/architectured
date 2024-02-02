use kernel::interfaces::journal::DependOnPersonManipulationEventJournal;
use crate::services::PersonCommandExecutionService;

impl<T> PersonCommandExecutionService for T
    where T: DependOnPersonManipulationEventJournal {}