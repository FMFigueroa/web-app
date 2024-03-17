use crate::{
    database::users::{self, Entity as Users},
    queires::user_queries::{find_by_username, save_active_user},
    utils::{
        app_error::AppError,
        hash::{hash_password, verify_password},
        jwt::create_token,
        token_wrapper::TokenWrapper,
    },
};
use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, Path, State},
    http::{Request, StatusCode},
    BoxError, Extension, Json, RequestExt,
};

use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: String,
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

#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: Option<String>,
}

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>,
    user: RequestUser,
) -> Result<Json<ResponseUser>, AppError> {
    let new_user = users::ActiveModel {
        username: Set(user.username.clone()),
        password: Set(hash_password(&user.password)?),
        token: Set(Some(create_token(&jwt_secret.0, user.username)?)),
        ..Default::default()
    }
    .save(&db)
    .await
    .map_err(|error| {
        let error_message = error.to_string();

        if error_message
            .contains("duplicate key value violates unique constraint \"users_username_key\"")
        {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "Username already taken, try again with a different user name",
            )
        } else {
            eprintln!("Error creating user: {:?}", error_message);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again",
            )
        }
    })?;

    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap(),
    }))
}

pub async fn get_one_user(
    Path(user_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let user = Users::find_by_id(user_id).one(&db).await.unwrap();
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
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseUser>>, StatusCode> {
    let users = Users::find()
        .all(&db)
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

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, AppError> {
    let user = find_by_username(&db, request_user.username).await?;

    if !verify_password(&request_user.password, &user.password)? {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "incorrect username and/or password",
        ));
    }

    let new_token = create_token(&jwt_secret.0, user.username.clone())?;
    let mut user = user.into_active_model();

    user.token = Set(Some(new_token));

    let user = save_active_user(&db, user).await?;

    let response = ResponseUser {
        id: user.id,
        username: user.username,
        token: user.token,
    };

    Ok(Json(response))
}

pub async fn logout(
    Extension(user): Extension<users::Model>,
    State(db): State<DatabaseConnection>,
) -> Result<StatusCode, AppError> {
    let mut user = user.into_active_model();

    user.token = Set(None);

    save_active_user(&db, user).await?;

    Ok(StatusCode::OK)
}

/* fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(|error| {
        eprintln!("Error verifying password: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "The was a problem verifying your password",
        )
    })
} */

/* fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, 14).map_err(|error| {
        eprintln!("Error hashing password: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error securing password")
    })
} */
