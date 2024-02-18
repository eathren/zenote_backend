use sqlx::PgPool;
use crate::models::graph::{Graph, NewGraphRequest};
use uuid::Uuid;

/// Inserts a new graph into the database.
pub async fn create_graph_db(pool: &PgPool, input: NewGraphRequest) -> Result<Graph, sqlx::Error> {
    let graph = sqlx::query_as!(
        Graph,
        "INSERT INTO graphs (name, owner_id) VALUES ($1, $2) RETURNING id, name, date_created, date_updated, deleted, owner_id",
        &input.name,
        input.owner_id
    )
    .fetch_one(pool)
    .await?;
    
    Ok(graph)
}

/// Fetches a graph by its ID from the database.
pub async fn fetch_graph_db(pool: &PgPool, graph_id: Uuid) -> Result<Graph, sqlx::Error> {
    let graph = sqlx::query_as!(
        Graph,
        "SELECT id, name, date_created, date_updated, deleted, owner_id FROM graphs WHERE id = $1",
        graph_id
    )
    .fetch_one(pool)
    .await?;
    
    Ok(graph)
}

/// Fetches all graphs from the database.
pub async fn fetch_all_graphs_db(pool: &PgPool) -> Result<Vec<Graph>, sqlx::Error> {
    let graphs = sqlx::query_as!(
        Graph,
        "SELECT id, name, date_created, date_updated, deleted, owner_id FROM graphs"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(graphs)
}

/// Deletes a graph by its ID from the database.
pub async fn delete_graph_db(pool: &PgPool, graph_id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM graphs WHERE id = $1",
        graph_id
    )
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected())
}