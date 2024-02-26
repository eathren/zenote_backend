use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;


#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub sub: String,
    pub email: String,
    pub date_created: Option<OffsetDateTime>,
    pub date_updated: Option<OffsetDateTime>,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NewUserRequest {
    pub email: String,
    pub sub: String,
}


#[derive(Serialize, Deserialize)]
pub struct UserUpdateRequest {
    pub email: Option<String>,
    pub deleted: bool,
}