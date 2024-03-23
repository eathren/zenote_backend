use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
enum Permission {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(Serialize, Deserialize, Debug)]
enum Role {
    Viewer,
    Editor,
    Admin,
}

#[derive(Serialize, Deserialize, Debug)]
struct GraphRoleAssignment {
    graph_id: Uuid,
    user_id: Uuid,
    role: Role,
}