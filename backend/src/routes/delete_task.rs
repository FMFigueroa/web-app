use crate::{
    database::users::Model,
    queires::task_queries::{find_task_by_id, save_active_task},
    utils::app_error::AppError,
};
use axum::{
    extract::{Path, State},
    Extension,
};
use chrono::Utc;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};

pub async fn delete_task(
    State(db): State<DatabaseConnection>, Path(task_id): Path<i32>,
    Extension(user): Extension<Model>,
) -> Result<(), AppError> {
    let mut task = find_task_by_id(&db, task_id, user.id)
        .await?
        .into_active_model();

    let now = Utc::now();

    task.deleted_at = Set(Some(now.into()));

    save_active_task(&db, task).await?;

    Ok(())
}
