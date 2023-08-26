// task routes
mod create_task;
mod delete_task;
mod get_tasks;
mod hello_world;
mod partial_update_task;
mod update_tasks;

// users routes
mod middleware_user_session;
mod partial_update_user;
mod users;

// essentials routes
mod mirror_body_json;
mod mirror_body_string;

use axum::{
    extract::FromRef,
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};

use create_task::create_task;
use delete_task::delete_task;
use get_tasks::{get_all_tasks, get_one_task};
use hello_world::hello_world;
use middleware_user_session::user_session;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use partial_update_task::partial_update;
use partial_update_user::partial_update_user;
use sea_orm::DatabaseConnection;
use update_tasks::atomic_update;
use users::{create_user, get_all_users, get_one_user, login, logout};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
}

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let app_state = AppState { database };
    Router::new()
        .route("/users/logout", post(logout))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            user_session,
        ))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
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
        .with_state(app_state)
}
