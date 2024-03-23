use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub common: CommonFields
}