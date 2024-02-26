
use axum::extract::State;

// shared middleware data between routes
pub async fn middleware_message(State(message): State<String>) -> String {
    message
}