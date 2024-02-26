mod hello_world;
mod mirror_body_json;
mod mirror_body_string;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_headers;
mod middleware_message;
mod read_middleware_custom_header;
mod set_middleware_custom_header;
mod always_errors;
mod returns_201;
mod get_json;

use axum::{
    extract::FromRef, middleware, routing::{get, post}, Router
};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::{path_variables, hard_coded_path};
use query_params::query_params;
use mirror_user_agent::mirror_user_agent;
use mirror_custom_headers::mirror_custom_header;
use middleware_message::middleware_message;
use read_middleware_custom_header::read_middleware_custom_header;
use set_middleware_custom_header::set_middleware_custom_header;
use always_errors::always_errors;
use returns_201::returns_201;
use get_json::get_json;
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone, FromRef)]
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
        .route("/read_middleware_custom_header", get(read_middleware_custom_header))
        .layer(middleware::from_fn(set_middleware_custom_header))
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
        .with_state(shared_data)
        .route("/always_errors", get(always_errors))
        .route("/returns_201", post(returns_201))
        .route("/get_json", get(get_json))
}
