use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};

use crate::{
    database::{
        tasks::{self, Model as TaskModel},
        users::Model as UserModel,
    },
    routes::create_task::ValidateCreateTask,
    utils::app_error::AppError,
};

pub async fn create_task(
    task: ValidateCreateTask,
    user: &UserModel,
    db: &DatabaseConnection,
) -> Result<TaskModel, AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title.unwrap()),
        description: Set(task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    save_active_task(db, new_task).await
}

pub async fn save_active_task(
    db: &DatabaseConnection,
    task: tasks::ActiveModel,
) -> Result<TaskModel, AppError> {
    let task = task.save(db).await.map_err(|error| {
        eprintln!("Error saving task: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving task")
    })?;

    convert_active_to_model(task)
}

fn convert_active_to_model(active_task: tasks::ActiveModel) -> Result<TaskModel, AppError> {
    active_task.try_into_model().map_err(|error| {
        eprintln!("Error converting task active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
