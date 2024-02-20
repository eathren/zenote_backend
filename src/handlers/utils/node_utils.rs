use sqlx::PgPool;
use crate::models::node::{Node, UpdateNodeRequest};
use uuid::Uuid;

pub async fn create_node_db(pool: &PgPool, graph_id: Uuid, name: String) -> Result<Node, sqlx::Error> {
    let node = sqlx::query_as!(
        Node,
        "INSERT INTO nodes (graph_id, name) VALUES ($1, $2) RETURNING *",
        graph_id,
        name,
    )
    .fetch_one(pool)
    .await?;

    Ok(node)
}

pub async fn fetch_node_db(pool: &PgPool, node_id: Uuid) -> Result<Node, sqlx::Error> {
    let node = sqlx::query_as!(
        Node,
        "SELECT * FROM nodes WHERE id = $1",
        node_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(node)
}

pub async fn update_node_db(pool: &PgPool, node_id: Uuid, input: UpdateNodeRequest) -> Result<Node, sqlx::Error> {
    let node = sqlx::query_as!(
        Node,
        r#"
        UPDATE nodes 
        SET 
            name = COALESCE($1, name), 
            deleted = COALESCE($2, deleted)
        WHERE id = $3 
        RETURNING *"#,
        input.name,
        input.deleted,
        node_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(node)
}

pub async fn fetch_all_nodes_db(pool: &PgPool, graph_id: Uuid) -> Result<Vec<Node>, sqlx::Error> {
   let nodes =  sqlx::query_as!(
        Node,
        "SELECT * FROM nodes WHERE deleted = false AND graph_id = $1",
        graph_id
    )
    .fetch_all(pool)
    .await?;

    Ok(nodes)
}

pub async fn delete_node_db(pool: &PgPool, node_id: Uuid) -> Result<u64, sqlx::Error> {
    sqlx::query_as!(
        Node,
        "DELETE FROM nodes WHERE id = $1",
        node_id
    )
    .execute(pool)
    .await
    .map(|result| result.rows_affected())
}