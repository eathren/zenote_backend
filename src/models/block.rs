use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct ListProperties {
    items: Vec<String>, 
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BlockType {
    List,
    Text,
    Page,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub id: Uuid,
    pub block_type: BlockType,
    pub properties: Value, // Flexible JSON structure for varying properties
    pub content: Vec<Uuid>, // Array of block IDs    
    pub parent_id: Option<Uuid>, // Nullable for root blocks
    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
}