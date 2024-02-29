use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::edge_handlers::{create_edge, fetch_all_edges, fetch_edge};

pub fn edge_routes() -> Router {
    Router::new()
        .route("/edges/:graph_id", get(fetch_all_edges))
        .route("/edge", post(create_edge))
        .route("/edge/:id", get(fetch_edge))
}
