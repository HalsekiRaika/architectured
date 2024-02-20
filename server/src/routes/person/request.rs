use serde::{Deserialize, Serialize};
use crate::Intake;

#[derive(Debug, Deserialize)]
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