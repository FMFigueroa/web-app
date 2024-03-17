use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonResponse {
    username: String,
    password: String,
    github: Option<String>,
}

pub async fn validate_json(
    Json(user): Json<JsonResponse>,
) -> Json<JsonResponse> {
    //println!("{user:?}");
    dbg!(&user);

    Json(JsonResponse {
        username: user.username,
        password: user.password,
        github: if user.github == None {
            Some("https://github.com/FMFigueroa".to_owned())
        } else {
            user.github
        },
    })
}
