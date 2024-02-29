use crate::models::user::User;
use sqlx::PgPool;

pub async fn create_user_db(pool: &PgPool, id: String, email: String) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (id, email) VALUES ($1, $2) RETURNING *",
        id,
        email,
    )
    .fetch_one(pool)
    .await
}

pub async fn fetch_user_db(pool: &PgPool, user_id: String) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id,)
        .fetch_one(pool)
        .await
}

pub async fn delete_user_db(pool: &PgPool, user_id: String) -> Result<u64, sqlx::Error> {
    sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(pool)
        .await
        .map(|result| result.rows_affected())
}

pub async fn fetch_all_users_db(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users",)
        .fetch_all(pool)
        .await
}
