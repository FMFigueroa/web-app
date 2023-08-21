use crate::database::users::{self, Entity as Users};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: Option<String>,
}

pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        password: Set(request_user.password),
        token: Set(Some("L_#=$w5265wAFrs98ETdkjs:".to_owned())),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap(),
    }))
}

pub async fn get_one_user(
    Path(user_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let user = Users::find_by_id(user_id).one(&database).await.unwrap();
    if let Some(user) = user {
        Ok(Json(ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_users(
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ResponseUser>>, StatusCode> {
    let users = Users::find()
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_user| ResponseUser {
            id: db_user.id,
            username: db_user.username,
            token: db_user.token,
        })
        .collect();

    Ok(Json(users))
}
