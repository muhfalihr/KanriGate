use axum::{
    extract::{Query, State},
    http::{header, StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    Json,
};
use crate::{
    models::{KanriGateResp, UsernameQuery, RoleBindingQuery, ClusterRoleBindingQuery, K8sConfigQuery},
    state::AppState,
    services::kubernetes::KubeOps,
    config::permissions,
};
use std::time::Instant;

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

pub async fn get_templates() -> impl IntoResponse {
    let start = Instant::now();
    let templates = permissions::TEMPLATES.to_vec();
    json_response(start, templates)
}

pub async fn get_namespaces(State(state): State<AppState>) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client);
    match kube.get_namespaces().await {
        Ok(ns) => json_response(start, ns).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

pub async fn get_service_accounts(State(state): State<AppState>) -> Response {
    let start = Instant::now();
    let kube = KubeOps::new(state.client);
    match kube.get_service_accounts().await {
        Ok(sas) => json_response(start, sas).into_response(),
        Err(e) => error_response(start, e).into_response(),
    }
}

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
            headers.insert(
                header::CONTENT_DISPOSITION, 
                "attachment; filename=kube-config.yaml".parse().unwrap()
            );
            (headers, config).into_response()
        },
        Err(e) => error_response(start, e).into_response(),
    }
}
