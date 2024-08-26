// Main

use axum::routing::get;
use axum::routing::post;
use axum::Extension;
use axum::Router;
use controller::task;
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tokio_postgres::{Client, NoTls};
use tower_http::services::ServeDir;
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
        .route("/list", get(task::handler_get_list))
        .route("/update/:task_id", post(task::handler_update))
        .route("/delete/:task_id", post(task::handler_delete))
        .route("/", get(task::handler_home))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(Extension(app_state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
