use error_stack::ResultExt;
use kernel::interfaces::journal::{DependOnPersonManipulationEventJournal, PersonManipulationEventJournal};
use kernel::prelude::commands::{PersonManipulationCommand, Publication};
use kernel::prelude::entities::PersonId;
use crate::error::ApplicationError;

#[async_trait::async_trait]
#[orbital::export_service]
pub trait PersonCommandExecutionService: 'static + Sync + Send
    + DependOnPersonManipulationEventJournal
{
    async fn execute(&self, cmd: PersonManipulationCommand) -> Result<(), error_stack::Report<ApplicationError>> {
        match cmd {
            PersonManipulationCommand::Create { .. } => {
                let ev = cmd.publish()
                    .change_context_lazy(|| ApplicationError::Kernel)?;

                self.person_manipulation_event_journal()
                    .create(&ev)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            PersonManipulationCommand::Rename { id, .. } => {
                let id = PersonId::new(id);
                let ev = cmd.publish()
                    .change_context_lazy(|| ApplicationError::Kernel)?;

                self.person_manipulation_event_journal()
                    .append(&id, &ev)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
        }

        Ok(())
    }
}