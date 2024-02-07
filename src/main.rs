use axum::{
    Router,
    Extension
};
use log::info;
mod db;
mod routes;
use sqlx::migrate::Migrator;
use routes::graph_routes;
use std::net::SocketAddr;
pub mod handlers;

// Static migrator
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
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
    .merge(graph_routes()) 
    .layer(Extension(pool)); 
    info!("Server started");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
