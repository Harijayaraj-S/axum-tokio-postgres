// controllers

use axum::{extract::Path, Extension, Json};
use serde::{Deserialize, Serialize};

use crate::{
    api::task::{self, CreateTask, Task, TaskUpdateInput},
    AppState,
};

use super::ControllerError;

#[derive(Serialize, Deserialize)]
pub struct RestResponse {
    success: bool,
}

pub async fn handler_create(
    Extension(app_state): Extension<AppState>,
    Json(input): Json<CreateTask>,
) -> Result<Json<RestResponse>, ControllerError> {
    let db = app_state.db;
    let success = task::create_task(db, input).await?;
    Ok(Json(RestResponse { success }))
}

pub async fn handler_get_list(
    Extension(app_state): Extension<AppState>,
) -> Result<Json<Vec<Task>>, ControllerError> {
    let db = app_state.db;
    let item = task::get_list(db).await?;
    Ok(Json(item))
}

pub async fn handler_update(
    Extension(app_state): Extension<AppState>,
    Json(input): Json<TaskUpdateInput>,
    Path(task_id): Path<i64>,
) -> Result<Json<RestResponse>, ControllerError> {
    let db = app_state.db;
    let success = task::update(db, task_id, input).await?;
    Ok(Json(RestResponse { success }))
}
