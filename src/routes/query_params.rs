use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryParams {
    id: i32,
}
#[derive(Serialize)]
pub struct JsonResponse {
    id: i32,
    username: String,
    message: String,
    github: String,
}

pub async fn query_params(Query(query): Query<QueryParams>) -> Json<JsonResponse> {
    if query.id == 1 {
        Json(JsonResponse {
            id: query.id,
            username: "Felix Manuel".to_owned(),
            message: "Rust Developer".to_owned(),
            github: "https://github.com/FMFigueroa".to_owned(),
        })
    } else {
        Json(JsonResponse {
            id: query.id,
            username: "user no found".to_owned(),
            message: "".to_owned(),
            github: "".to_owned(),
        })
    }
}
