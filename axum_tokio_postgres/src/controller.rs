// controllers

use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

use crate::{
    api::{self, CreateTask, Task},
    AppState,
};

#[derive(Serialize, Deserialize)]
pub struct RestResponse {
    success: bool,
}

pub async fn handler_create(
    Extension(app_state): Extension<AppState>,
    Json(input): Json<CreateTask>,
) -> Json<RestResponse> {
    let db = app_state.db;
    let success = api::create_task(db, input).await;
    Json(RestResponse { success })
}

pub async fn handler_get_list(Extension(app_state): Extension<AppState>) -> Json<Vec<Task>> {
    let db = app_state.db;
    let item = api::get_list(db).await;
    Json(item)
}
