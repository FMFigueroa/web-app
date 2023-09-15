use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    count: i32,
    username: String,
    message: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        count: 2023,
        username: "felix".to_owned(),
        message: "https://github.com/FMFigueroa".to_owned(),
    };

    Json(data)
}
