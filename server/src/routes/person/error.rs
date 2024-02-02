use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonManipulationError {
    Create
}

impl IntoResponse for PersonManipulationError {
    fn into_response(self) -> Response {
        serde_json::to_string(&self)
            .expect("")
            .into_response()
    }
}