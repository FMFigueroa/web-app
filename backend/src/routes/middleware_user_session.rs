use crate::{
    database::users::{self, Entity as Users},
    utils::{
        app_error::AppError, jwt::validate_token, token_wrapper::TokenWrapper,
    },
};
use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    TypedHeader,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn user_session<T>(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    State(database): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>, mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let token = token.token().to_owned();
    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token.clone())))
        .one(&database)
        .await
        .map_err(|_error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
        })?;
    validate_token(&jwt_secret.0, &token)?; // Validating token after getting from the database to obsfucate that the token is wrong. Feel free to move up if you are not worried about that.

    let Some(user) = user else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized, please login or create account",
        ));
    };

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
