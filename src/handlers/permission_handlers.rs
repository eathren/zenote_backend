
// this seems wrongs
pub fn role_permissions(role: &Role) -> Vec<Permission> {
    match role {
        Role::Viewer => vec![Permission::Read],
        Role::Editor => vec![Permission::Read, Permission::Update, Permission::Create],
        Role::Admin => vec![Permission::Read, Permission::Update, Permission::Create, Permission::Delete],
    }
}
pub fn user_has_permission(
    user_id: &Uuid,
    graph_id: &Uuid,
    permission: &Permission,
    assignments: &[GraphRoleAssignment],
) -> bool {
    assignments.iter().any(|assignment| {
        assignment.user_id == *user_id
            && assignment.graph_id == *graph_id
            && role_permissions(&assignment.role).contains(permission)
    })
}