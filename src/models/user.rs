use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
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
