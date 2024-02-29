use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::graph_handlers::{create_graph, fetch_all_graphs, fetch_graph};

pub fn graph_routes() -> Router {
    Router::new()
        .route("/graphs", get(fetch_all_graphs))
        .route("/graph", post(create_graph))
        .route("/graph/:id", get(fetch_graph))
}
