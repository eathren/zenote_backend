use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub graph_id: Uuid,
    pub date_created: DateTime<Utc>,
    pub date_updated: DateTime<Utc>,
    pub deleted: bool,
}
