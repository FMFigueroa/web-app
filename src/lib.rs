use app_state::AppState;
use std::net::SocketAddr;

pub mod app_state;
mod database;
mod routes;
pub mod utils;

pub async fn run(app_state: AppState) {
    let app = routes::create_routes(app_state).await;
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
