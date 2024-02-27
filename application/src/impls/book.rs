use kernel::interfaces::io::DependOnAcquireTransaction;
use kernel::interfaces::journal::DependOnBookEventJournal;
use crate::service::BookCommandExecutionService;

impl<T> BookCommandExecutionService for T
    where T: DependOnBookEventJournal
           + DependOnAcquireTransaction {}