use serde::{Deserialize, Serialize};
use uuid::Uuid;

use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct Graph {
    pub id: Uuid,
    pub name: String,
    pub date_created: Option<OffsetDateTime>,
    pub date_updated: Option<OffsetDateTime>,
    pub deleted: bool,
    pub owner_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct NewGraphRequest {
    pub name: String,
    pub owner_id: Uuid,
}

