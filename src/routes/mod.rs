mod create_task;
mod get_one_task;
mod hello_world;

use axum::{
    routing::{get, post},
    Extension, Router,
};

use create_task::create_task;
use get_one_task::get_one_task;
use hello_world::hello_world;
use sea_orm::DatabaseConnection;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/tasks", post(create_task))
        .route("/tasks/:task_id", get(get_one_task))
        .layer(Extension(database))
}
