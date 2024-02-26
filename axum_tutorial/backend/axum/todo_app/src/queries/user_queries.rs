use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::{database::users, utils::app_error::AppError};

pub async fn save_active_user(db: &DatabaseConnection, user: users::ActiveModel) -> Result<users::ActiveModel, AppError> {
    user.save(db).await.map_err(|e| {
        let error_message = e.to_string();

        if error_message
            .contains("duplicate key value violates unique constraint \"users_username_key\"")
        {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "Username already taken, try again with a different username",
            )
        } else {
            eprintln!("Error creating user: {:?}", error_message);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again",
            )
        }
    })
}