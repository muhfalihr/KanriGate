use axum::{
    extract::{Query, State},
    http::{header, StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use crate::{
    models::{LoginRequest, AuthBody, KanriGateResp, UsernameQuery, RoleBindingQuery, ClusterRoleBindingQuery, K8sConfigQuery},
    state::AppState,
    services::kubernetes::KubeOps,
    config::permissions,
};
use std::time::Instant;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// Helper to create response
fn json_response<T>(start: Instant, data: T) -> Json<KanriGateResp<T>> {
    let duration = start.elapsed().as_secs_f64();
    Json(KanriGateResp::new(200, "OK", duration, data))
}

fn error_response(start: Instant, err: anyhow::Error) -> (StatusCode, Json<KanriGateResp<String>>) {
    let duration = start.elapsed().as_secs_f64();
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(KanriGateResp::new(500, err.to_string(), duration, String::new())),
    )
}

#[utoipa::path(
    post,
    path = "/apps/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = KanriGateRespAuthBody),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Response {
    let start = Instant::now();
    
    // 1. Check username
    if payload.username != state.config.admin_username {
        return (
            StatusCode::UNAUTHORIZED,
            Json(KanriGateResp::new(401, "Invalid credentials", start.elapsed().as_secs_f64(), String::new()))
        ).into_response();
    }

    // 2. Verify password hash
    let parsed_hash = match PasswordHash::new(&state.config.admin_password_hash) {
        Ok(h) => h,
        Err(e) => return error_response(start, anyhow::anyhow!("Invalid password hash config: {}", e)).into_response(),
    };

    if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_err() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(KanriGateResp::new(401, "Invalid credentials", start.elapsed().as_secs_f64(), String::new()))
        ).into_response();
    }

    // 3. Generate JWT
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: payload.username,
        exp: expiration as usize,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_ref()),
    ) {
        Ok(t) => t,
        Err(e) => return error_response(start, anyhow::anyhow!("Token generation failed: {}", e)).into_response(),
    };

    json_response(start, AuthBody {
        access_token: token,
        token_type: "Bearer".to_string(),
    }).into_response()
}

#[utoipa::path(
    get,
    path = "/apps/getTemplates",
    responses(
        (status = 200, description = "Success", body = KanriGateRespVecString)
    )
)]
pub async fn get_templates() -> impl IntoResponse {
    let start = Instant::now();
    let templates = permissions::TEMPLATES.to_vec();
    json_response(start, templates)
}

#[utoipa::path(
    get,
    path = "/apps/getNamespaces",
    responses(
        (status = 200, description = "Success", body = KanriGateRespVecString)
    )
)]
pub async fn get_namespaces(State(state): State<AppState>) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client);
    match kube.get_namespaces().await {
        Ok(ns) => json_response(start, ns).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/apps/getServiceAccounts",
    responses(
        (status = 200, description = "Success", body = KanriGateRespVecString)
    )
)]
pub async fn get_service_accounts(State(state): State<AppState>) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client);
    match kube.get_service_accounts().await {
        Ok(sas) => json_response(start, sas).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/apps/createServiceAccount",
    params(UsernameQuery),
    responses(
        (status = 200, description = "Success", body = KanriGateRespString)
    )
)]
pub async fn create_service_account(
    State(state): State<AppState>,
    Query(query): Query<UsernameQuery>,
) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client);
    match kube.create_service_account(&query.username).await {
        Ok(name) => json_response(start, name).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/apps/deleteServiceAccount",
    params(UsernameQuery),
    responses(
        (status = 200, description = "Success", body = KanriGateRespString)
    )
)]
pub async fn delete_service_account(
    State(state): State<AppState>,
    Query(query): Query<UsernameQuery>,
) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client);
    match kube.delete_service_account(&query.username).await {
        Ok(name) => json_response(start, name).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/apps/createSecret",
    params(UsernameQuery),
    responses(
        (status = 200, description = "Success", body = KanriGateRespString)
    )
)]
pub async fn create_secret(
    State(state): State<AppState>,
    Query(query): Query<UsernameQuery>,
) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client);
    match kube.create_secret(&query.username).await {
        Ok(name) => json_response(start, name).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/apps/deleteSecret",
    params(UsernameQuery),
    responses(
        (status = 200, description = "Success", body = KanriGateRespString)
    )
)]
pub async fn delete_secret(
    State(state): State<AppState>,
    Query(query): Query<UsernameQuery>,
) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client);
    match kube.delete_secret(&query.username).await {
        Ok(names) => json_response(start, names).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/apps/createRoleBinding",
    params(RoleBindingQuery),
    responses(
        (status = 200, description = "Success", body = KanriGateRespString)
    )
)]
pub async fn create_role_binding(
    State(state): State<AppState>,
    Query(query): Query<RoleBindingQuery>,
) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client);
    match kube.create_role_binding(&query.username, &query.namespace, &query.permission).await {
        Ok(name) => json_response(start, name).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/apps/deleteRoleBinding",
    params(RoleBindingQuery),
    responses(
        (status = 200, description = "Success", body = KanriGateRespString)
    )
)]
pub async fn delete_role_binding(
    State(state): State<AppState>,
    Query(query): Query<RoleBindingQuery>,
) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client);
    match kube.delete_role_binding(&query.username, &query.namespace, &query.permission).await {
        Ok(name) => json_response(start, name).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/apps/createClusterRoleBinding",
    params(ClusterRoleBindingQuery),
    responses(
        (status = 200, description = "Success", body = KanriGateRespString)
    )
)]
pub async fn create_cluster_role_binding(
    State(state): State<AppState>,
    Query(query): Query<ClusterRoleBindingQuery>,
) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client);
    match kube.create_cluster_role_binding(&query.username, &query.permission).await {
        Ok(name) => json_response(start, name).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/apps/deleteClusterRoleBinding",
    params(ClusterRoleBindingQuery),
    responses(
        (status = 200, description = "Success", body = KanriGateRespString)
    )
)]
pub async fn delete_cluster_role_binding(
    State(state): State<AppState>,
    Query(query): Query<ClusterRoleBindingQuery>,
) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client);
    match kube.delete_cluster_role_binding(&query.username, &query.permission).await {
        Ok(name) => json_response(start, name).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/apps/getFilteredRoleBindings",
    params(UsernameQuery),
    responses(
        (status = 200, description = "Success", body = KanriGateRespVecString)
    )
)]
pub async fn get_filtered_role_bindings(
    State(state): State<AppState>,
    Query(query): Query<UsernameQuery>,
) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client);
    match kube.get_filtered_role_bindings(&query.username).await {
        Ok(data) => json_response(start, data).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/apps/getFilteredClusterRoleBindings",
    params(UsernameQuery),
    responses(
        (status = 200, description = "Success", body = KanriGateRespVecString)
    )
)]
pub async fn get_filtered_cluster_role_bindings(
    State(state): State<AppState>,
    Query(query): Query<UsernameQuery>,
) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client);
    match kube.get_filtered_cluster_role_bindings(&query.username).await {
        Ok(data) => json_response(start, data).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/apps/generateK8sConfig",
    params(K8sConfigQuery),
    responses(
        (status = 200, description = "Success", body = KanriGateRespString)
    )
)]
pub async fn generate_k8s_config(
    State(state): State<AppState>,
    Query(query): Query<K8sConfigQuery>,
) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client.clone());
    match kube.generate_k8s_config(
        &query.username, 
        &query.namespace, 
        &state.config.cluster_name,
        &state.config.control_plane_address
    ).await {
        Ok(config) => json_response(start, config).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/apps/generateK8sConfigDownloadFile",
    params(K8sConfigQuery),
    responses(
        (status = 200, description = "Success", body = String)
    )
)]
pub async fn generate_k8s_config_download(
    State(state): State<AppState>,
    Query(query): Query<K8sConfigQuery>,
) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client.clone());
    match kube.generate_k8s_config(
        &query.username, 
        &query.namespace, 
        &state.config.cluster_name,
        &state.config.control_plane_address
    ).await {
        Ok(config) => {
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, "application/x-yaml".parse().unwrap());
            let filename = format!("kubeconfig-{}-{}.yaml", query.username, query.namespace);
            headers.insert(
                header::CONTENT_DISPOSITION, 
                format!("attachment; filename=\"{}\"", filename).parse().unwrap()
            );
            (headers, config).into_response()
        },
        Err(e) => error_response(start, e).into_response(),
    }
}
