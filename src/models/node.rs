use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub graph_id: Uuid,
    pub date_created: Option<OffsetDateTime>,
    pub date_updated: Option<OffsetDateTime>,
    pub deleted: bool,
}
