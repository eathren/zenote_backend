use crate::models::user::User;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user_db(pool: &PgPool, user_id: Uuid, email: String) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (user_id, email) VALUES ($1, $2) RETURNING *",
        user_id,
        email,
    )
    .fetch_one(pool)
    .await
}

pub async fn fetch_user_db(pool: &PgPool, user_id: Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE user_id = $1", user_id,)
        .fetch_one(pool)
        .await
}

pub async fn delete_user_db(pool: &PgPool, user_id: Uuid) -> Result<u64, sqlx::Error> {
    sqlx::query!("DELETE FROM users WHERE user_id = $1", user_id)
        .execute(pool)
        .await
        .map(|result| result.rows_affected())
}

pub async fn fetch_all_users_db(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users",)
        .fetch_all(pool)
        .await
}
