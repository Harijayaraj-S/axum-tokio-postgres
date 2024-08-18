// controller

pub mod task;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::api::AppError;

#[derive(Error, Debug)]
pub enum ControllerError {
    #[error("{0}")]
    AppError(#[from] AppError),
}

impl IntoResponse for ControllerError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", self),
        )
            .into_response()
    }
}
