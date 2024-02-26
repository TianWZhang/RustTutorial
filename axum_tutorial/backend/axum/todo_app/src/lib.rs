pub mod app_state;
mod database;
mod middleware;
mod queries;
mod routes;
pub mod utils;

use app_state::AppState;
use routes::create_routes;

pub async fn run(app_state: AppState) {
    let app = create_routes(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
