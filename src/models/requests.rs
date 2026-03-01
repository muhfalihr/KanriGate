use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoParams};

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct UsernameQuery {
    pub username: String,
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct RoleBindingQuery {
    pub username: String,
    pub namespace: String,
    pub permission: String,
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct ClusterRoleBindingQuery {
    pub username: String,
    pub permission: String,
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct K8sConfigQuery {
    pub username: String,
    pub namespace: String,
}

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
}

#[derive(Serialize, ToSchema)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String
}