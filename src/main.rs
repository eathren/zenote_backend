
use log::info;
mod db;
mod routes;
use std::net::SocketAddr;
pub mod handlers;
use axum::{
    Router,
    Extension,
    error_handling::HandleErrorLayer,
};
use tower::ServiceBuilder;
use std::time::Duration;
mod errors;
use errors::handle_generic_error;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting server");

    info!("Checking for .env file");
    dotenv::dotenv().ok().expect("Failed to load .env file");
    info!("env file found");

    let pool = db::establish_connection().await?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    info!("Migrations run");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
    .merge(routes::graph_routes()) 
    .merge(routes::user_routes())
    .merge(routes::node_routes())
    .merge(routes::edge_routes())
    .layer(Extension(pool))
    .layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_generic_error)) // Handle errors generically
            .timeout(Duration::from_secs(30))

    );

    info!("Server started");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

