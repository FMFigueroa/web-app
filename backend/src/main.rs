use dotenvy::dotenv;
use eyre::Result;
use sea_orm::Database;
use web_app::{app_state::AppState, run, utils::token_wrapper::TokenWrapper};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    // create app state variables
    let database_url = dotenvy::var("DATABASE_URL")?;
    let jwt_secret = dotenvy::var("JWT_SECRET")?;

    // connect database
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };

    // create Appstate
    let app_state = AppState {
        db,
        jwt_secret: TokenWrapper(jwt_secret),
    };
    run(app_state).await?;

    Ok(())
}
