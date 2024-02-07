pub enum BlockType {
    List,
    Text,
    // Add other block types as needed
}

pub struct Block {
    pub id: i32,
    pub name: String,
    pub node_id: i32,
    pub date_created: String,
    pub date_updated: String,
    pub deleted: bool,
    pub owner_id: i32,
    pub block_type: BlockType,
}