use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::CommonFields;

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub graph_id: Uuid,
    pub common: CommonFields
}

#[derive(Serialize, Deserialize)]
pub struct NewNodeRequest {
    pub graph_id: Uuid,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateNodeRequest {
    pub name: Option<String>,
    pub deleted: Option<bool>,
}
