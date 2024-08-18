use axum::routing::post;
use axum::Extension;
use axum::Router;
use controller::task;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};
mod api;
mod controller;

#[derive(Clone)]
struct AppState {
    db: Arc<Client>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let client: Arc<Client> = Arc::new(client);
    let app_state = AppState { db: client };
    let app = Router::new()
        .route("/create", post(task::handler_create))
        .route("/list", post(task::handler_get_list))
        .route("/update/:task_id", post(task::handler_update))
        .layer(Extension(app_state));

    let addr = "127.0.0.1:8000".parse().unwrap();
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}