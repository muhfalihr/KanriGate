use serde::Deserialize;

#[derive(Deserialize)]
pub struct UsernameQuery {
    pub username: String,
}

#[derive(Deserialize)]
pub struct RoleBindingQuery {
    pub username: String,
    pub namespace: String,
    pub permission: String,
}

#[derive(Deserialize)]
pub struct ClusterRoleBindingQuery {
    pub username: String,
    pub permission: String,
}

#[derive(Deserialize)]
pub struct K8sConfigQuery {
    pub username: String,
    pub namespace: String,
}
