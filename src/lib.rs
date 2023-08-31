use app_state::AppState;
use eyre::Result;
use std::net::SocketAddr;

pub mod app_state;
mod database;
mod queires;
mod routes;
pub mod utils;

pub async fn run(app_state: AppState) -> Result<()> {
    let app = routes::create_routes(app_state).await;

    // region: ---Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> LISTENING on http://{addr}\n");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    // endregion: ---Start Server
    Ok(())
}
