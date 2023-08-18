use dotenvy::dotenv;
use web_app::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenvy::var("DATABASE_URL").unwrap();
    run(&database_uri).await;
}
