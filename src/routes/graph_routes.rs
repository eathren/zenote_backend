use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::graph_handlers::{ fetch_all_graphs};

pub fn graph_routes() -> Router {
    Router::new()
        // .route("/graphs", post(create_graph))
        .route("/graphs", get(fetch_all_graphs))
        // .route("/graphs/:id", get(fetch_all_graphs))
}
