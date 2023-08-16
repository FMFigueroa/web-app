use dotenvy::dotenv;
use web_app::run;
use web_app::connect;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenvy::var("DATABASE_URL").unwrap();
    connect(database_uri).await;
    run().await;
}
