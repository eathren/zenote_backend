
use log::info;
mod db;
mod routes;
use std::net::SocketAddr;
pub mod handlers;
use axum::{
    error_handling::HandleErrorLayer, middleware, Extension, Router
};
use tower::ServiceBuilder;
use std::time::Duration;
mod errors;
use errors::handle_generic_error;
use crate::db::migrate_databases;
use tower_http::cors::{CorsLayer, Origin};
mod models;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting server");

    info!("Checking for .env file");
    dotenv::dotenv().ok().expect("Failed to load .env file");
    info!("env file found");

    let pool = db::establish_connection().await?;
    migrate_databases().await?;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let allowed_origins = AllowOrigin::list(vec![
        "http://localhost:3000".parse().unwrap(),
        "https://zenote.net".parse().unwrap(),
    ]);

    let cors = CorsLayer::new()
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
        .allow_headers(vec!["Content-Type", "Authorization"])
        .allow_origin(allowed_origins)
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    // All current routes declared here
    let api_v1_routes = Router::new()
    .nest("/users", routes::user_routes())
    .nest("/graphs", routes::graph_routes())
    .nest("/nodes", routes::node_routes())
    .nest("/edges", routes::edge_routes());

    let app = Router::new()
    .nest("/api/v1/", api_v1_routes)
    .layer(Extension(pool))
    .layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_generic_error)) 
            .timeout(Duration::from_secs(30))
    )
    .layer(cors);

    info!("Server started");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

