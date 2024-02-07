use axum::{
    routing::get,
    Router,
};
use log::info;
mod db;
use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting server");

    info!("Checking for .env file");
    dotenv::dotenv().ok().expect("Failed to load .env file");
    info!("env file found");

    let pool = db::establish_connection().await?;

    MIGRATOR.run(&pool).await?;
    info!("Migrations run");

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    info!("Server started");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("Listening");
    axum::serve(listener, app).await?;
    Ok(())
}
