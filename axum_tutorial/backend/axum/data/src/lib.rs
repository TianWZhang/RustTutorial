mod database;
mod routes;
mod utils;

use sea_orm::Database;

pub async fn run(db_uri: &str) {
    let database = Database::connect(db_uri).await.unwrap();
    let app = routes::create_routes(database);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
