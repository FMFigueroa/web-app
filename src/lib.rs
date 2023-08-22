use sea_orm::Database;

mod database;
mod routes;
mod utils;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();
    // build our application with a single route
    let app = routes::create_routes(database).await;

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
