use crate::database::prelude::Users;
use crate::database::users;
use crate::utils::app_error::AppError;
use crate::utils::jwt::verify_jwt;
use axum::{extract::{Request, State}, http::StatusCode, middleware::Next, response::Response};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn guard(State(db): State<DatabaseConnection>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    mut request: Request, next: Next) -> Result<Response, AppError> {
    let token = token
        .token()
        .to_owned();
    // validate the token from the request
    verify_jwt(&token)?;

    let user = Users::find()
        .filter(users::Column::Token.eq(token))
        .one(&db)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))?;
    let Some(user) = user else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized, please log in or create an account",
        ));
    };
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
