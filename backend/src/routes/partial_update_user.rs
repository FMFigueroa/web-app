/*
** Partial Updates Users
*/
use crate::database::users::{self, Entity as Users};
use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, Path, State},
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: Option<String>,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: Option<String>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
}
#[async_trait]
impl<S, B> FromRequest<S, B> for RequestUser
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(request: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = request
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;

        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }

        Ok(user)
    }
}

pub async fn partial_update_user(
    Path(user_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    user: RequestUser,
) -> Result<(), StatusCode> {
    let mut db_user = if let Some(user) = Users::find_by_id(user_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    if let Some(username) = user.username {
        db_user.username = Set(username);
    }

    if let Some(password) = user.password {
        db_user.password = Set(hash_password(password)?);
    }

    if let Some(deleted_at) = user.deleted_at {
        db_user.deleted_at = Set(deleted_at);
    }

    Users::update(db_user)
        .filter(users::Column::Id.eq(user_id))
        .exec(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 14).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}
