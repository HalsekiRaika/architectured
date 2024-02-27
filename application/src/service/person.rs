use error_stack::{Report, ResultExt};
use kernel::interfaces::io::{AcquireTransaction, DependOnAcquireTransaction, Transaction};
use kernel::interfaces::journal::{BookEventJournal, DependOnBookEventJournal, DependOnPersonManipulationEventJournal, PersonManipulationEventJournal};
use kernel::prelude::commands::{PersonManipulationCommand, Publish};
use kernel::prelude::entities::{BookId, PersonId};
use kernel::prelude::events::{BookEvent, PersonEvent};
use crate::error::ApplicationError;

#[async_trait::async_trait]
#[orbital::export_service]
pub trait PersonCommandExecutionService: 'static + Sync + Send
    + DependOnAcquireTransaction
    + DependOnPersonManipulationEventJournal
    + DependOnBookEventJournal
{
    async fn execute(&self, cmd: PersonManipulationCommand) -> Result<(), Report<ApplicationError>> {
        let mut transaction = self.acquire_transaction().acquire().await
            .change_context_lazy(|| ApplicationError::Driver)?;
        
        match cmd {
            PersonManipulationCommand::Create { .. } => {
                let ev = Publish::<PersonEvent>::publish(cmd)
                    .change_context_lazy(|| ApplicationError::Kernel)?;
                
                self.person_manipulation_event_journal()
                    .create(&ev, &mut transaction)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
                
                transaction.commit().await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            PersonManipulationCommand::Rename { id, .. } => {
                let id = PersonId::new(id);
                let ev = Publish::<PersonEvent>::publish(cmd)
                    .change_context_lazy(|| ApplicationError::Kernel)?;

                self.person_manipulation_event_journal()
                    .append(&id, &ev, &mut transaction)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
                
                transaction.commit().await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            PersonManipulationCommand::Rental { id, book } => {
                let id = PersonId::new(id);
                let book = BookId::new(book);
                let (ev1, ev2) = Publish::<(PersonEvent, BookEvent)>::publish(cmd)
                    .change_context_lazy(|| ApplicationError::Kernel)?;
                
                self.person_manipulation_event_journal()
                    .append(&id, &ev1, &mut transaction)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
                
                self.book_event_journal()
                    .append(&book, &ev2, &mut transaction)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
                
                transaction.commit().await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            PersonManipulationCommand::Return { id, book } => {
                let id = PersonId::new(id);
                let book = BookId::new(book);
                let (ev1, ev2) = Publish::<(PersonEvent, BookEvent)>::publish(cmd)
                    .change_context_lazy(|| ApplicationError::Kernel)?;
                
                self.person_manipulation_event_journal()
                    .append(&id, &ev1, &mut transaction)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
                
                self.book_event_journal()
                    .append(&book, &ev2, &mut transaction)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
                
                transaction.commit().await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
        }

        Ok(())
    }
}