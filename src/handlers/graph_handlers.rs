use axum::{
    extract::{Extension, Path},
    Json, response::{IntoResponse, Response},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::graph::NewGraphRequest;
use super::utils::graph_utils::{create_graph_db, delete_graph_db, fetch_all_graphs_db, fetch_graph_db};

/// Handler for creating a graph
pub async fn create_graph(
    Extension(pool): Extension<PgPool>,
    Json(input): Json<NewGraphRequest>,
) -> Response {
    match create_graph_db(&pool, input).await {
        Ok(graph) => (StatusCode::CREATED, Json(graph)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create graph").into_response(),
    }
}

/// Handler for fetching a graph by ID
pub async fn fetch_graph(
    Extension(pool): Extension<PgPool>,
    Path(graph_id): Path<Uuid>,
) -> Response {
    match fetch_graph_db(&pool, graph_id).await {
        Ok(graph) => (StatusCode::OK, Json(graph)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Graph not found").into_response(),
    }
}

/// Handler for fetching all graphs
// this should be user specific.
pub async fn fetch_all_graphs(
    Extension(pool): Extension<PgPool>,
) -> Response {
    match fetch_all_graphs_db(&pool).await {
        Ok(graphs) => (StatusCode::OK, Json(graphs)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch graphs").into_response(),
    }
}

/// Handler for deleting a graph by ID
pub async fn delete_graph(
    Extension(pool): Extension<PgPool>,
    Path(graph_id): Path<Uuid>,
) -> Response {
    match delete_graph_db(&pool, graph_id).await {
        Ok(rows) if rows > 0 => StatusCode::OK.into_response(),
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}