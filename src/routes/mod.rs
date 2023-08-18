use axum::{
    routing::{get, post},
    Extension, Router,
};

use create_task::create_task;
use hello_world::hello_world;
use sea_orm::DatabaseConnection;

mod create_task;
mod hello_world;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/tasks", post(create_task))
        .layer(Extension(database))
}
