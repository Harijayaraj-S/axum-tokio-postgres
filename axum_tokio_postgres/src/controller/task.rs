// controllers

use axum::{extract::Path, response::Html, Extension, Json};
use sailfish::TemplateOnce;
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

#[derive(TemplateOnce)]
#[template(path = "list.stpl")]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

pub async fn handler_get_list(
    Extension(app_state): Extension<AppState>,
) -> Result<Html<String>, ControllerError> {
    let db = app_state.db;
    let tasks = task::get_list(db).await?;
    let ctx: TaskList = TaskList { tasks };

    Ok(axum::response::Html(ctx.render_once().unwrap()))
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

pub async fn handler_delete(
    Extension(app_state): Extension<AppState>,
    Path(task_id): Path<i64>,
) -> Result<Json<RestResponse>, ControllerError> {
    let db = app_state.db;
    let success = task::delete(db, task_id).await?;
    Ok(Json(RestResponse { success }))
}
