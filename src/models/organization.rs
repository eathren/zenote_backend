use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub date_created: Option<OffsetDateTime>,
    pub date_updated: Option<OffsetDateTime>,
    pub deleted: Option<bool>,
    pub date_deleted: Option<OffsetDateTime>,
}


#[derive(Deserialize, Serialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateOrganizationRequest {
    pub name: String,
}