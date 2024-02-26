use super::{RequestUser, ResponseDataUser, ResponseUser};
use crate::database::users;
use crate::queries::user_queries;
use crate::utils::app_error::AppError;
use crate::utils::hash::hash_password;
use crate::utils::jwt::create_jwt;
use crate::utils::token_wrapper::TokenWrapper;
use axum::extract::State;
use axum::Json;
use sea_orm::{DatabaseConnection, Set};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let new_user = users::ActiveModel {
        username: Set(request_user.username.clone()),
        password: Set(hash_password(request_user.password)?),
        token: Set(Some(create_jwt(&jwt_secret.0, request_user.username)?)),
        ..Default::default()
    };
    let user = user_queries::save_active_user(&db, new_user).await?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            username: user.username.unwrap(),
            id: user.id.unwrap(),
            token: user.token.unwrap().unwrap(),
        },
    }))
}
