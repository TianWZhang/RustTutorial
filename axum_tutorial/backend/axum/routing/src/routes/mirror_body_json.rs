use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MirrorJson {
    message: String
}

#[derive(Serialize, Deserialize)]
pub struct MirrorJsonResponse {
    message: String,
    server_message: String
}

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    Json(MirrorJsonResponse {
        message: body.message,
        server_message: "Hello from Axum".to_owned()
    })
}
