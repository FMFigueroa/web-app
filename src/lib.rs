use routes::create_routes;
use sea_orm::Database;

mod routes;

pub async fn run() {
    // build our application with a single route
    let app = create_routes();

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn connect(database_uri:String){
    let database = Database::connect(database_uri).await;
}