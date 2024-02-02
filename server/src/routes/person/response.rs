use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use kernel::prelude::entities::{PersonId, PersonName};
use crate::Exhaust;

#[derive(Debug, Serialize)]
pub struct CreatedResponse {
    id: PersonId,
    name: PersonName
}

pub struct Presenter;

impl Exhaust<kernel::prelude::events::PersonManipulationEvent> for Presenter {
    type To = CreatedResponse;
    fn emit(&self, input: kernel::prelude::events::PersonManipulationEvent) -> Self::To {
        match input {
            kernel::prelude::events::PersonManipulationEvent::Created { id, name } => Self::To {
                id, name
            },

            _ => unreachable!()
        }
    }
}


impl IntoResponse for CreatedResponse {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}