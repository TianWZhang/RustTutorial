use crate::utils::app_error::AppError;
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(30);
    now += expires_in;
    let exp = now.timestamp() as usize;
    let claim = Claims { exp, iat };

    let secret = dotenv!("JWT_SECRET");
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), &claim, &encoding_key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
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
