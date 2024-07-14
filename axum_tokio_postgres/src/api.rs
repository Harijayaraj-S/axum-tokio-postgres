use std::sync::Arc;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

#[derive(Serialize, Deserialize)]
pub struct Task {
    id: i64,
    title: String,
    priority: String,
    created_on: String,
    is_done: bool,
    description: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub priority: String,
    pub description: Option<String>,
}

pub async fn create_task(db: Arc<Client>, input: CreateTask) -> bool {
    let id = db
        .execute(
            "INSERT INTO task (title, priority, description, is_done)  
            VALUES ($1, $2, $3, $4) RETURNING id",
            &[&input.title, &input.priority, &input.description, &false],
        )
        .await
        .unwrap();
    id != 0
}

pub async fn get_list(db: Arc<Client>) -> Vec<Task> {
    let mut item: Vec<Task> = vec![];
    let rows = db
        .query("SELECT * FROM task", &[])
        .await
        .expect("Unable to get list");

    for row in rows {
        let created_on = row.get::<_,NaiveDateTime>("created_on").format("%Y-%m-%d %H:%M").to_string();
        item.push(Task {
            id: row.get("id"),
            title: row.get("title"),
            priority: row.get("priority"),
            created_on,
            is_done: row.get("is_done"),
            description: row.get("description"),
        })
    }
    item
}
