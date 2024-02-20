mod hello_world;
mod mirror_body_json;
mod mirror_body_string;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_headers;
mod middleware_message;

use axum::{
    routing::{get, post}, Extension, Router
};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::{path_variables, hard_coded_path};
use query_params::query_params;
use mirror_user_agent::mirror_user_agent;
use mirror_custom_headers::mirror_custom_header;
use middleware_message::middleware_message;
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    pub message: String
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
    // allow `GET` and `POST` when accessing the resource
    .allow_methods([Method::GET, Method::POST])
    // allow requests from any origin
    .allow_origin(Any);

    let shared_data = SharedData {message: "Hello from shared data".to_owned()};

    Router::new()
        .route("/hello_world", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(cors)
        .layer(Extension(shared_data))
}
