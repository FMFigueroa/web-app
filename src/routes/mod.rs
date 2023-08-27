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
mod middleware_message;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod mirror_user_agent;
mod path_variables;
mod query_params;
mod read_middleware_custom_header;
mod set_middleware_custom_header;

use axum::{
    extract::FromRef,
    http::Method,
    middleware,
    routing::{delete, get, patch, post, put},
    Extension, Router,
};

use create_task::create_task;
use delete_task::delete_task;
use get_tasks::{get_all_tasks, get_one_task};
use hello_world::hello_world;
use middleware_user_session::user_session;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use partial_update_task::partial_update;
use partial_update_user::partial_update_user;
use path_variables::{hard_coded_path, path_variables};
use query_params::query_params;
use sea_orm::DatabaseConnection;
use set_middleware_custom_header::set_middleware_custom_header;
use tower_http::cors::{Any, CorsLayer};
use update_tasks::atomic_update;
use users::{create_user, get_all_users, get_one_user, login, logout};

use self::{
    middleware_message::middleware_message,
    read_middleware_custom_header::read_middleware_custom_header,
};

#[derive(Clone, FromRef)]
pub struct SharedData {
    pub message: String,
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
}

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let app_state = AppState { database };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared data, I'm a State now".to_owned(),
    };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/users/logout", post(logout))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:task_id", get(get_one_task))
        .route("/tasks/:task_id", put(atomic_update))
        .route("/tasks/:task_id", patch(partial_update))
        .route("/tasks/:task_id", delete(delete_task))
        .route("/users", get(get_all_users))
        .route("/users/:user_id", get(get_one_user))
        .route("/users/:user_id", patch(partial_update_user))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            user_session,
        ))
        .route("/", get(hello_world))
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
        .with_state(app_state)
}
