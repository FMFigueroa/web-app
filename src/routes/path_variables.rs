use axum::extract::Path;

pub async fn path_variables(Path(id): Path<i32>) -> String {
    id.to_string()
}

pub async fn hard_coded_path() -> String {
    "You got 15...this route is a hard_coded_path!".to_owned()
}
