use axum::{
     routing::{delete, get, post, put},
    Router,
};

use crate::services::organization::{create_organization, delete_organization, fetch_organization, update_organization};


pub fn organization_routes() -> Router {
    Router::new()
        .route("/organization", post(create_organization))
        .route("/organization", get(fetch_organization))
        .route("/organization", put(update_organization))
        .route("/organization", delete(delete_organization))
}
