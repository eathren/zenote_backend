use axum::extract::{Extension, Path};
use axum::{Json, response::IntoResponse, http::StatusCode};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::graph::{Graph, NewGraphRequest};
use log::error;

pub async fn create_graph(
    Extension(pool): Extension<PgPool>,
    Json(input): Json<NewGraphRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query_as!(
        Graph,
        "INSERT INTO graphs (name, owner_id) VALUES ($1, $2) RETURNING id, name, date_created, date_updated, deleted, owner_id",
        &input.name,
        input.owner_id
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(graph) => Ok((StatusCode::CREATED, Json(graph))),
        Err(e) => {
            error!("Failed to create graph: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}
pub async fn fetch_all_graphs(
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query_as!(Graph,
        "SELECT * FROM graphs"
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(graphs) => Ok((StatusCode::OK, Json(graphs))),
        Err(e) => {
            error!("Failed to fetch graphs: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

pub async fn fetch_graph(
    Extension(pool): Extension<PgPool>,
    Path(graph_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query_as!(
        Graph,
        "SELECT * FROM graphs WHERE id = $1",
        graph_id,
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(graph) => Ok((StatusCode::OK, Json(graph))),
        Err(e) => {
            error!("Graph not found: {:?}", e);
            Err(StatusCode::NOT_FOUND) 
        },
    }
}