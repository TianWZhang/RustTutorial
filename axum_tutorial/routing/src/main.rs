use routing::routes::create_routes;

#[tokio::main]
async fn main() {
    let app = create_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
