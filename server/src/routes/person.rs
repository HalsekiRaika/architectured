#![allow(unused)]

mod error;
mod request;
mod response;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use application::services::{DependOnPersonCommandExecutionService, PersonCommandExecutionService};
use crate::{AppModule, Controller};

use self::request::{CreateRequest, Transformer};
use self::response::Presenter;
use self::error::PersonManipulationError;

pub async fn person(
    State(handler): State<AppModule>,
    Json(req): Json<CreateRequest>,
) -> Result<impl IntoResponse, PersonManipulationError> {
    // Controller::new(Transformer, Presenter)
    //     .intake(req)
    //     .handle(|cmd| async {
    //         handler.person_command_execution_service()
    //             .execute(cmd)
    //             .await
    //     })
    //     .await?;
    Ok(StatusCode::CREATED)
}
