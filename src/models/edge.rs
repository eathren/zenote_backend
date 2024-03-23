use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::CommonFields;

#[derive(Serialize, Deserialize)]
pub struct Edge {
    pub id: Uuid,
    pub label: Option<String>,
    pub graph_id: Uuid,
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub common: CommonFields
}

#[derive(Serialize, Deserialize)]
pub struct NewEdgeRequest {
    pub graph_id: Uuid,
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateEdgeRequest {
    pub label: Option<String>,
}
