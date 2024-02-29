use crate::handlers::user_handlers::{create_user, get_all_users, get_user};
use axum::{
    routing::{get, post},
    Router,
};

pub fn user_routes() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .route("/users", get(get_all_users))
}
