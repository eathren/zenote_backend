use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::User;

pub async fn create_user_db(pool: &PgPool, sub: String, email: String) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (sub, email) VALUES ($1, $2) RETURNING *",
        sub,
        email,
    )
    .fetch_one(pool)
    .await
}

pub async fn fetch_user_db(pool: &PgPool, user_id: Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        user_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_user_db(pool: &PgPool, user_id: Uuid) -> Result<u64, sqlx::Error> {
    sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        user_id
    )
    .execute(pool)
    .await
    .map(|result| result.rows_affected())
}
