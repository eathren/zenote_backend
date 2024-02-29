use hyper::Method;
use log::info;
mod db;
mod routes;
use std::{net::SocketAddr, str::FromStr};
pub mod handlers;
use axum::{error_handling::HandleErrorLayer, http::HeaderName, Extension, Router};
use std::time::Duration;
use tower::ServiceBuilder;
mod errors;
use crate::db::migrate_databases;
use errors::handle_generic_error;
use tower_http::cors::{AllowOrigin, CorsLayer};
mod models;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting server");

    info!("Checking for .env file");
    dotenv::dotenv().expect("Failed to load .env file");
    info!("env file found");

    let pool = db::establish_connection().await?;
    migrate_databases().await?;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let allowed_origins = AllowOrigin::list(vec![
        "http://localhost:3000".parse().unwrap(),
        "https://zenote.net".parse().unwrap(),
    ]);

    let cors = CorsLayer::new()
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(vec![
            HeaderName::from_str("Content-Type").unwrap(),
            HeaderName::from_str("Authorization").unwrap(),
        ])
        .allow_origin(allowed_origins)
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    // All current routes declared here
    let api_v1_routes = Router::new()
        .nest("/", routes::user_routes())
        .nest("/", routes::graph_routes())
        .nest("/", routes::node_routes())
        .nest("/", routes::edge_routes());

    let app = Router::new()
        .nest("/api/v1/", api_v1_routes)
        .layer(Extension(pool))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_generic_error))
                .timeout(Duration::from_secs(30))
                .layer(cors),
        );

    info!("Server started");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
