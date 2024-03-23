use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::CommonFields;

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph {
    pub id: Uuid,
    pub name: String,
    pub org_id: Option<Uuid>, 
    pub user_id: Option<Uuid>,
    pub common: CommonFields
}

#[derive(Serialize, Deserialize)]
pub struct NewGraphRequest {
    pub name: String,
    pub owner_id: String,
}
