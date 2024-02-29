use crate::models::edge::{Edge, NewEdgeRequest};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_edge_db(pool: &PgPool, input: NewEdgeRequest) -> Result<Edge, sqlx::Error> {
    let edge = sqlx::query_as!(
        Edge,
        "INSERT INTO edges (graph_id, source_id, target_id) VALUES ($1, $2, $3) RETURNING *",
        input.graph_id,
        input.source_id,
        input.target_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(edge)
}

pub async fn fetch_edge_db(pool: &PgPool, edge_id: Uuid) -> Result<Edge, sqlx::Error> {
    let edge = sqlx::query_as!(Edge, "SELECT * FROM edges WHERE id = $1", edge_id,)
        .fetch_one(pool)
        .await?;

    Ok(edge)
}

pub async fn fetch_all_edges_db(pool: &PgPool, graph_id: Uuid) -> Result<Vec<Edge>, sqlx::Error> {
    let edges = sqlx::query_as!(Edge, "SELECT * FROM edges WHERE graph_id = $1", graph_id)
        .fetch_all(pool)
        .await?;

    Ok(edges)
}

pub async fn delete_edge_db(pool: &PgPool, edge_id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM edges WHERE id = $1", edge_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

pub async fn fetch_edges_by_source_and_target_db(
    pool: &PgPool,
    graph_id: Uuid,
    source_id: Uuid,
    target_id: Uuid,
) -> Result<Vec<Edge>, sqlx::Error> {
    let edges = sqlx::query_as!(
        Edge,
        "SELECT * FROM edges WHERE graph_id = $1 AND source_id = $2 AND target_id = $3",
        graph_id,
        source_id,
        target_id
    )
    .fetch_all(pool)
    .await?;

    Ok(edges)
}

pub async fn fetch_edges_by_source_db(
    pool: &PgPool,
    graph_id: Uuid,
    source_id: Uuid,
) -> Result<Vec<Edge>, sqlx::Error> {
    let edges = sqlx::query_as!(
        Edge,
        "SELECT * FROM edges WHERE graph_id = $1 AND source_id = $2",
        graph_id,
        source_id
    )
    .fetch_all(pool)
    .await?;

    Ok(edges)
}

pub async fn fetch_edges_by_target_db(
    pool: &PgPool,
    graph_id: Uuid,
    target_id: Uuid,
) -> Result<Vec<Edge>, sqlx::Error> {
    let edges = sqlx::query_as!(
        Edge,
        "SELECT * FROM edges WHERE graph_id = $1 AND target_id = $2",
        graph_id,
        target_id
    )
    .fetch_all(pool)
    .await?;

    Ok(edges)
}
