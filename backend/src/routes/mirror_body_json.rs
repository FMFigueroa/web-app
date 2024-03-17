use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJson {
    profile: String,
    message: String,
}

#[derive(Serialize)]
pub struct MirrorJsonResponse {
    id: i32,
    profile: String,
    message: String,
    message_from_server: String,
    github: String,
}

pub async fn mirror_body_json(
    Json(body): Json<MirrorJson>,
) -> Json<MirrorJsonResponse> {
    Json(MirrorJsonResponse {
        id: 01,
        profile: body.profile,
        message: body.message,
        message_from_server: "Hello from Axum".to_owned(),
        github: "https://github.com/FMFigueroa".to_owned(),
    })
}
