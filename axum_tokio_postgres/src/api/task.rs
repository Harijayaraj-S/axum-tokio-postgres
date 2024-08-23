use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_postgres::Client;

use super::AppError;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub priority: String,
    pub created_on: String,
    pub is_done: bool,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TaskUpdateInput {
    title: String,
    priority: String,
    description: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub priority: String,
    pub description: Option<String>,
}

pub async fn create_task(db: Arc<Client>, input: CreateTask) -> Result<bool, AppError> {
    let id = db
        .execute(
            "INSERT INTO task (title, priority, description, is_done)  
            VALUES ($1, $2, $3, $4) RETURNING id",
            &[&input.title, &input.priority, &input.description, &false],
        )
        .await?;
    Ok(id != 0)
}

pub async fn get_list(db: Arc<Client>) -> Result<Vec<Task>, AppError> {
    let mut item: Vec<Task> = vec![];
    let rows = db
        .query("SELECT * FROM task", &[])
        .await
        .expect("Unable to get list");

    for row in rows {
        let created_on = row
            .get::<_, NaiveDateTime>("created_on")
            .format("%Y-%m-%d %H:%M")
            .to_string();
        item.push(Task {
            id: row.get("id"),
            title: row.get("title"),
            priority: row.get("priority"),
            created_on,
            is_done: row.get("is_done"),
            description: row.get("description"),
        })
    }
    Ok(item)
}

pub async fn update(
    db: Arc<Client>,
    task_id: i64,
    input: TaskUpdateInput,
) -> Result<bool, AppError> {
    let row = db
        .execute(
            "UPDATE task SET title = $1, priority = $2, description = $3 WHERE id = $4",
            &[&input.title, &input.priority, &input.description, &task_id],
        )
        .await?;

    Ok(row != 0)
}

pub async fn delete(db: Arc<Client>, task_id: i64) -> Result<bool, AppError> {
    let row = db
        .execute("DELETE FROM task WHERE id = $1", &[&task_id])
        .await?;

    Ok(row != 0)
}
