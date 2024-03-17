use crate::{
    database::users::Model as UserModel, queires::task_queries,
    utils::app_error::AppError,
};
use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, State},
    http::{Request, StatusCode},
    BoxError, Extension, Json, RequestExt,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct ValidateCreateTask {
    #[validate(length(min = 1, max = 1))]
    pub priority: Option<String>,
    #[validate(required(message = "missing task title"))]
    pub title: Option<String>,
    pub description: Option<String>,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for ValidateCreateTask
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(
        req: Request<B>, _state: &S,
    ) -> Result<ValidateCreateTask, Self::Rejection> {
        let Json(task) = req
            .extract::<Json<ValidateCreateTask>, _>()
            .await
            .map_err(|error| {
                eprintln!("Error extracting new task: {:?}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, please try again",
                )
            })?;

        if let Err(errors) = task.validate() {
            let field_errors = errors.field_errors();
            for (_, error) in field_errors {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    error.first().unwrap().clone().message.unwrap().to_string(), // feel safe unwrapping because we know there is at least one error, and we only care about the first for this api
                ));
            }
        }

        Ok(task)
    }
}

#[derive(Serialize)]
pub struct ResponseTask {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed_at: Option<String>,
    pub user_id: Option<i32>,
}

pub async fn create_task(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>, task: ValidateCreateTask,
) -> Result<(StatusCode, Json<ResponseTask>), AppError> {
    let task = task_queries::create_task(task, &user, &db).await?;

    Ok((
        StatusCode::CREATED,
        Json(ResponseTask {
            id: task.id,
            title: task.title,
            description: task.description,
            priority: task.priority,
            user_id: task.user_id,
            completed_at: task.completed_at.map(|time| time.to_string()),
        }),
    ))
}
