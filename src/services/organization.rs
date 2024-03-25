use crate::{db::organization::{create_organization_db, delete_organization_db, fetch_organization_db, update_organization_db}, models::organization::{CreateOrganizationRequest, UpdateOrganizationRequest}};
use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;


/// This function will create a new organization.
pub async fn create_organization(
    pool: Extension<PgPool>,
    Json(body): Json<CreateOrganizationRequest>,
) -> impl IntoResponse {
    let organization = create_organization_db(&pool, body).await;

    match organization {
        Ok(organization) => (StatusCode::CREATED, Json(organization)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal Server Error")).into_response(),
    }
}

/// This function will fetch an organization by its ID. 
pub async fn fetch_organization(
    pool: Extension<PgPool>,
    Query(organization_id): Query<Uuid>,
) -> impl IntoResponse {
    let organization = fetch_organization_db(&pool, organization_id).await;

    match organization {
        Ok(organization) => (StatusCode::OK, Json(organization)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal Server Error")).into_response(),
    }
}

/// This function will update the name of the organization.
pub async fn update_organization(
    pool: Extension<PgPool>,
    Query(organization_id): Query<Uuid>,
    body: Json<UpdateOrganizationRequest>,
) -> impl IntoResponse {
    let organization = update_organization_db(&pool, organization_id, body.name.clone()).await;

    match organization {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::OK
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// This function will delete an organization by its ID.
pub async fn delete_organization(
    pool: Extension<PgPool>,
    Query(organization_id): Query<Uuid>,
) -> impl IntoResponse {
    let organization = delete_organization_db(&pool, organization_id).await;

    match organization {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::OK
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
