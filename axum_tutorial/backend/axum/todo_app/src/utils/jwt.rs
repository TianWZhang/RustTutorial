use crate::utils::app_error::AppError;
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    exp: usize,
    username: String,
}

pub fn create_jwt(secret: &str, username: String) -> Result<String, AppError> {
    let expires_at = Utc::now() + Duration::hours(1);
    let exp = expires_at.timestamp() as usize;
    let claim = Claims { exp, username };
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), &claim, &encoding_key).map_err(|e| {
        eprintln!("Error creating token: {:?}", e);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error, please try again later",
        )
    })
}

pub fn verify_jwt(token: &str) -> Result<bool, AppError> {
    let secret = dotenv!("JWT_SECRET");
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256)).map_err(
        |e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::new(
                StatusCode::UNAUTHORIZED,
                "Your session has expired, please log in again",
            ),
            _ => AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again",
            ),
        },
    )?;
    Ok(true)
}
