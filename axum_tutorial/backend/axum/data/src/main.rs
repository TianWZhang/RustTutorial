use dotenvy::dotenv;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_uri = dotenv!("DATABASE_URL");
    data::run(db_uri).await;
}
