use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub graph_id: Uuid,
    pub date_created: Option<OffsetDateTime>,
    pub date_updated: Option<OffsetDateTime>,
    pub deleted: bool,
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
