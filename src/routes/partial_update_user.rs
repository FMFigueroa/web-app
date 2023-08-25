/*
** Partial Updates Users
*/
use crate::database::users::{self, Entity as Users};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestUser {
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub token: Option<Option<String>>,
    pub username: Option<String>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
}

pub async fn partial_update_user(
    Path(user_id): Path<i32>,
    State(database): State<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<(), StatusCode> {
    let mut db_user = if let Some(user) = Users::find_by_id(user_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    if let Some(username) = request_user.username {
        db_user.username = Set(username);
    }

    if let Some(token) = request_user.token {
        db_user.token = Set(token);
    }

    if let Some(deleted_at) = request_user.deleted_at {
        db_user.deleted_at = Set(deleted_at);
    }

    Users::update(db_user)
        .filter(users::Column::Id.eq(user_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
