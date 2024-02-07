use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Edge {
    pub id: Uuid,
    pub label: String,
    pub source_id: Uuid,
    pub target_id: Uuid,
}
