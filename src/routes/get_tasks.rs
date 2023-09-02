use crate::{
    database::{
        tasks::{self, Entity as Tasks},
        users::Model,
    },
    queires::task_queries::find_task_by_id,
    utils::app_error::AppError,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::{DateTime, FixedOffset};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
pub struct GetTasksQueryParams {
    priority: Option<String>,
    user_id: Option<i32>,
}

pub async fn get_all_tasks(
    Query(query_params): Query<GetTasksQueryParams>,
    State(database): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let mut priority_filter = Condition::all();
    if let Some(priority) = query_params.priority {
        priority_filter = if priority.is_empty() {
            priority_filter.add(tasks::Column::Priority.is_null())
        } else {
            priority_filter.add(tasks::Column::Priority.eq(priority))
        };
    }

    let mut user_id_filter = Condition::all();
    if let Some(user_id) = query_params.user_id {
        user_id_filter = user_id_filter.add(tasks::Column::UserId.eq(user_id));
    }

    let tasks = Tasks::find()
        .filter(priority_filter)
        .filter(user_id_filter)
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
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
        .collect();

    Ok(Json(tasks))
}
