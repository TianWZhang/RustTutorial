use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sea_orm::Database;
use todo_app::{app_state::AppState, run, utils::token_wrapper::TokenWrapper};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_uri = dotenv!("DATABASE_URL");
    let jwt_secret = dotenv!("JWT_SECRET").to_owned();

    let db = match Database::connect(db_uri).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };
    let app_state = AppState {
        db,
        jwt_secret: TokenWrapper(jwt_secret),
    };
    run(app_state).await;
}
