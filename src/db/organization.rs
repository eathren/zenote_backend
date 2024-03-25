use sqlx::PgPool;
use uuid::Uuid;
use crate::models::organization::{CreateOrganizationRequest, Organization};
/// Inserts a new organization into the database.
/// Returns the newly created organization.
/// If the organization already exists, it will return an error.
/// If the organization is successfully created, it will return the organization.
/// If there is an error, it will return the error.
pub async fn create_organization_db(pool: &PgPool, input: CreateOrganizationRequest) -> Result<Organization, sqlx::Error> {
    let organization = sqlx::query_as!(
        Organization,
        "INSERT INTO organizations (name) VALUES ($1) RETURNING id, name, date_created, date_updated, deleted, date_deleted",
        &input.name
    )
    .fetch_one(pool)
    .await?;

    Ok(organization)
}

/// Fetches an organization by its ID from the database.
/// Returns the organization.
/// If the organization does not exist, it will return an error.
/// If the organization has been successfully fetched, it will return the organization.
/// If there is an error, it will return the error.
/// If the organization has been deleted, it will return an error.
pub async fn fetch_organization_db(pool: &PgPool, organization_id: Uuid) -> Result<Organization, sqlx::Error> {
    let organization = sqlx::query_as!(
        Organization,
        "SELECT id, name, date_created, date_updated, deleted, date_deleted FROM organizations WHERE id = $1",
        organization_id
    )
    .fetch_one(pool)
    .await?;

    Ok(organization)
}

/// Updates an organization by its ID in the database.
/// This function will update the name of the organization.
/// Returns the number of rows affected.
/// If the organization does not exist, it will return 0.
/// If the organization has been successfully updated, it will return 1.
/// If there is an error, it will return the error.
/// If the organization has been deleted, it will return an error.
pub async fn update_organization_db(pool: &PgPool, organization_id: Uuid, name: String) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE organizations SET name = $1 WHERE id = $2",
        name,
        organization_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

/// Deletes an organization by its ID from the database.
/// This function will set the `deleted` field to `true` and set the `date_deleted` field to the current timestamp.
/// Returns the number of rows affected.
/// If the organization does not exist, it will return 0.
/// If the organization has already been deleted, it will return 0.
/// If the organization has been successfully deleted, it will return 1.
pub async fn delete_organization_db(pool: &PgPool, organization_id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE organizations SET deleted = true, date_deleted = now() WHERE id = $1 AND deleted = false",
        organization_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}