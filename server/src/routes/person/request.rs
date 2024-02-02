use serde::Serialize;
use crate::Intake;

#[derive(Debug, Serialize)]
pub struct CreateRequest {
    name: String
}

pub struct Transformer;

impl Intake<CreateRequest> for Transformer {
    type To = kernel::prelude::commands::PersonManipulationCommand;
    fn emit(&self, input: CreateRequest) -> Self::To {
        Self::To::Create {
            name: input.name,
        }
    }
}