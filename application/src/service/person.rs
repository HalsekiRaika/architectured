use error_stack::{Report, ResultExt};
use kernel::interfaces::io::{AcquireTransaction, DependOnAcquireTransaction, Transaction};
use kernel::interfaces::journal::{DependOnPersonManipulationEventJournal, PersonManipulationEventJournal};
use kernel::prelude::commands::{PersonManipulationCommand, Publication};
use kernel::prelude::entities::PersonId;
use crate::error::ApplicationError;

#[async_trait::async_trait]
#[orbital::export_service]
pub trait PersonCommandExecutionService: 'static + Sync + Send
    + DependOnAcquireTransaction
    + DependOnPersonManipulationEventJournal
{
    async fn execute(&self, cmd: PersonManipulationCommand) -> Result<(), Report<ApplicationError>> {
        let mut transaction = self.acquire_transaction().acquire().await
            .change_context_lazy(|| ApplicationError::Driver)?;
        
        match cmd {
            PersonManipulationCommand::Create { .. } => {
                let ev = cmd.publish()
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
                let ev = cmd.publish()
                    .change_context_lazy(|| ApplicationError::Kernel)?;

                self.person_manipulation_event_journal()
                    .append(&id, &ev, &mut transaction)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
        }

        Ok(())
    }
}