mod atomic_update_task;
mod create_task;
mod custom_json_extractor;
mod delete_task;
mod get_tasks;
mod guard_middleware;
mod partial_update_task;
mod user_handler;

use atomic_update_task::atomic_update;
use axum::{
    extract::FromRef, middleware, routing::{delete, get, patch, post, put}, Router
};
use create_task::create_task;
use custom_json_extractor::custom_json_extractor;
use delete_task::delete_task;
use get_tasks::{get_all_tasks, get_one_task};
use guard_middleware::guard;
use partial_update_task::partial_update;
use sea_orm::DatabaseConnection;
use user_handler::{create_user, login, logout};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection
}

pub fn create_routes(database: DatabaseConnection) -> Router {
    let app_state = AppState {database};
    Router::new()
        .route("/users/logout", post(logout))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), guard))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .route("/tasks/:task_id", get(get_one_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:task_id", put(atomic_update))
        .route("/tasks/:task_id", patch(partial_update))
        .route("/tasks/:task_id", delete(delete_task))
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .with_state(app_state)
}
