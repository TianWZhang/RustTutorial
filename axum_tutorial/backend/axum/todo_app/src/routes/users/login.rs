use super::{RequestUser, ResponseDataUser, ResponseUser};
use crate::database::{prelude::Users, users};
use crate::queries::user_queries::save_active_user;
use crate::utils::{
    app_error::AppError, hash::verify_password, jwt::create_jwt, token_wrapper::TokenWrapper,
};
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let db_user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&db)
        .await
        .map_err(|e| {
            eprintln!("Error getting user by username: {:?}", e);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error logging in, please try again later",
            )
        })?;
    if let Some(db_user) = db_user {
        if !verify_password(request_user.password, &db_user.password)? {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "incorrect username and/or password",
            ));
        }
        let new_token = create_jwt(&jwt_secret.0, db_user.username.clone())?;
        let mut user = db_user.into_active_model();
        user.token = Set(Some(new_token));
        let saved_user = save_active_user(&db, user).await?;
        Ok(Json(ResponseDataUser {
            data: ResponseUser {
                username: saved_user.username.unwrap(),
                id: saved_user.id.unwrap(),
                token: saved_user.token.unwrap().unwrap(),
            },
        }))
    } else {
        Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "incorrect username and/or password",
        ))
    }
}
