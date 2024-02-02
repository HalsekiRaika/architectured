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
    Json(req): Json<CreateRequest>,
    State(handler): State<AppModule>
) -> Result<StatusCode, PersonManipulationError> {
    // Controller::new(Transformer, Presenter)
    //     .intake(req)
    //     .handle(|cmd| async {
    //         handler.person_command_execution_service()
    //             .execute(cmd)
    //             .await
    //     })
    //     .await?;
    todo!()
}

pub async fn person_act() -> impl IntoResponse {

}