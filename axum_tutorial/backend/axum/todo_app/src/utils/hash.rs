use super::app_error::AppError;
use axum::http::StatusCode;

const COST: u32 = 12;

pub fn hash_password(password: String) -> Result<String, AppError> {
    bcrypt::hash(password, COST).map_err(|e| {
        eprintln!("Error hashing password: {:?}", e);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error hashing password")
    })
}

pub fn verify_password(password: String, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash).map_err(|e| {
        eprintln!("Error verifying password: {:?}", e);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was a problem verifying your password",
        )
    })
}
