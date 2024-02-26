use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    Json, RequestExt,
};

use serde::Deserialize;
use validator::Validate;

use crate::utils::app_error::AppError;

#[derive(Deserialize, Debug, Validate)]
pub struct ValidateCreateTask {
    #[validate(length(min = 1, max = 1, message = "must be a single character"))]
    pub priority: Option<String>,
    #[validate(required(message = "missing task title"))]
    pub title: Option<String>,
    pub description: Option<String>,
}

#[async_trait]
impl<S> FromRequest<S> for ValidateCreateTask
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(task) = req
            .extract::<Json<ValidateCreateTask>, _>()
            .await
            .map_err(|e| {
                eprintln!("Error extracting new task: {:?}", e);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, please try again",
                )
            })?;

        if let Err(e) = task.validate() {
            let field_errors = e.field_errors();
            for (_, error) in field_errors {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    // feel safe unwrapping because we know there is at least one error, and we only
                    // care about the first for this api
                    error.first().unwrap().clone().message.unwrap().to_string(),
                ));
            }
        }

        Ok(task)
    }
}
