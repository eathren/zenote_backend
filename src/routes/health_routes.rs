use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::health_handlers::health_check;

pub fn health_routes() -> Router {
    Router::new().route("/health", get(health_check))
}
