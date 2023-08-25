use crate::database::tasks;
use crate::database::users::{self, Entity as Users};
use axum::extract::State;
use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Json, TypedHeader,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    title: String,
    priority: Option<String>,
    description: Option<String>,
}

pub async fn create_task(
    authorization: TypedHeader<Authorization<Bearer>>,
    State(database): State<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let token = authorization.token();

    let user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_task = tasks::ActiveModel {
        title: Set(request_task.title),
        priority: Set(request_task.priority),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    let result = new_task.save(&database).await.unwrap();

    dbg!(result);

    Ok(())
}
