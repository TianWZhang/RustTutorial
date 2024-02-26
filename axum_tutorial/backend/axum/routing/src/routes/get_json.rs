use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonData {
    message: String,
    count: i32,
    username: String
}

pub async fn get_json() -> Json<JsonData> {
    let data = JsonData {
        message: "I am in data".to_owned(),
        count: 6753,
        username: "zhang tianwei".to_owned()
    };
    Json(data)
}