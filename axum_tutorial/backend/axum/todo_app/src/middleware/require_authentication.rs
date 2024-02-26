use crate::database::users;
use crate::utils::app_error::AppError;
use crate::utils::jwt::verify_jwt;
use crate::{database::prelude::Users, utils::token_wrapper::TokenWrapper};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn require_authentication(
    State(db): State<DatabaseConnection>,
    State(token_secret): State<TokenWrapper>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = token.token().to_owned();
    // validate the token from the request
    verify_jwt(&token_secret.0, &token)?;

    let user = Users::find()
        .filter(users::Column::Token.eq(token))
        .one(&db)
        .await
        .map_err(|e| {
            eprintln!("Error getting user by token: {:?}", e);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was a problem getting your account",
            )
        })?;

    if let Some(user) = user {
        request.extensions_mut().insert(user);
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized, please log in or create an account",
        ));
    };

    Ok(next.run(request).await)
}
