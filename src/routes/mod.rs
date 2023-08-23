mod create_task;
mod delete_task;
mod get_tasks;
mod hello_world;
mod partial_update_task;
mod partial_update_user;
mod update_tasks;
mod users;

use axum::{
    routing::{delete, get, patch, post, put},
    Extension, Router,
};

use create_task::create_task;
use delete_task::delete_task;
use get_tasks::{get_all_tasks, get_one_task};
use hello_world::hello_world;
use partial_update_task::partial_update;
use partial_update_user::partial_update_user;
use sea_orm::DatabaseConnection;
use update_tasks::atomic_update;
use users::{create_user, get_all_users, get_one_user, login, logout};

pub async fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:task_id", get(get_one_task))
        .route("/tasks/:task_id", put(atomic_update))
        .route("/tasks/:task_id", patch(partial_update))
        .route("/tasks/:task_id", delete(delete_task))
        .route("/users", post(create_user))
        .route("/users", get(get_all_users))
        .route("/users/:user_id", get(get_one_user))
        .route("/users/:user_id", patch(partial_update_user))
        .route("/users/login", post(login))
        .route("/users/logout", post(logout))
        .layer(Extension(database))
}
