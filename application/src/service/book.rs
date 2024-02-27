use error_stack::{Report, ResultExt};
use kernel::interfaces::io::{AcquireTransaction, DependOnAcquireTransaction, Transaction};
use kernel::interfaces::journal::{BookEventJournal, DependOnBookEventJournal};
use kernel::prelude::commands::{BookCommand, Publish};
use kernel::prelude::entities::BookId;
use kernel::prelude::events::BookEvent;
use crate::errors::ApplicationError;

#[orbital::export_service]
#[async_trait::async_trait]
pub trait BookCommandExecutionService: 'static + Sync + Send
    + DependOnAcquireTransaction
    + DependOnBookEventJournal
{
    async fn execute(&self, cmd: BookCommand) -> Result<(), Report<ApplicationError>> {
        let mut transaction = self.acquire_transaction().acquire().await
            .change_context_lazy(|| ApplicationError::Driver)?;
        
        match cmd { 
            BookCommand::Arrive { .. } => {
                let ev = Publish::<BookEvent>::publish(cmd)
                    .change_context_lazy(|| ApplicationError::Kernel)?;
                self.book_event_journal()
                    .create(&ev, &mut transaction)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
                transaction.commit().await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            BookCommand::Discard { id, .. } => {
                let id = BookId::new(id);
                let ev = Publish::<BookEvent>::publish(cmd)
                    .change_context_lazy(|| ApplicationError::Kernel)?;
                self.book_event_journal()
                    .append(&id, &ev, &mut transaction)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
                transaction.commit().await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
        }
        
        Ok(())
    }
}