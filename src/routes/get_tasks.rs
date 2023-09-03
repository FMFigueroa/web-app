use crate::{
    database::users::Model,
    queires::task_queries::{find_all_tasks, find_task_by_id},
    utils::app_error::AppError,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::{DateTime, FixedOffset};
use sea_orm::DatabaseConnection;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    priority: Option<String>,
    description: Option<String>,
    completed_at: Option<String>,
    user_id: Option<i32>,
    deleted_at: Option<DateTime<FixedOffset>>,
}

#[derive(Serialize)]
pub struct ResponseDataTasks {
    pub data: Vec<ResponseTask>,
}

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(StatusCode, Json<ResponseTask>), AppError> {
    let task = find_task_by_id(&db, task_id, user.id).await?;

    Ok((
        StatusCode::OK,
        Json(ResponseTask {
            id: task.id,
            title: task.title,
            description: task.description,
            priority: task.priority,
            completed_at: task.completed_at.map(|time| time.to_string()),
            user_id: task.user_id,
            deleted_at: task.deleted_at,
        }),
    ))
}

pub async fn get_all_tasks(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(StatusCode, Json<ResponseDataTasks>), AppError> {
    let tasks = find_all_tasks(&db, user.id, false)
        .await?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            description: db_task.description,
            priority: db_task.priority,
            completed_at: db_task.completed_at.map(|time| time.to_string()),
            user_id: db_task.user_id,
            deleted_at: db_task.deleted_at,
        })
        .collect::<Vec<ResponseTask>>();

    Ok((StatusCode::OK, Json(ResponseDataTasks { data: tasks })))
}
