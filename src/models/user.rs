use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::CommonFields;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub is_individual: bool,
    pub common: CommonFields
}

#[derive(Serialize, Deserialize)]
pub struct NewUserRequest {
    pub email: String,
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct UserUpdateRequest {
    pub email: Option<String>,
}
