mod users;

use self::users::create_user::create_user;
use crate::app_state::AppState;
use axum::{routing::post, Router};

pub fn create_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/v1/users", post(create_user))
        .with_state(app_state)
}
