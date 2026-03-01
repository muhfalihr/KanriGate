use axum::{
    routing::{get, post, delete},
    Router,
    middleware,
};
use tower_http::trace::TraceLayer;
use tower_http::cors::CorsLayer;
use crate::state::AppState;
use crate::api::{handlers, auth};

pub fn app_router(state: AppState) -> Router {
    // Routes that require authentication
    let protected_routes = Router::new()
        .route("/getTemplates", get(handlers::get_templates))
        .route("/getNamespaces", get(handlers::get_namespaces))
        .route("/getServiceAccounts", get(handlers::get_service_accounts))
        .route("/getFilteredRoleBindings", post(handlers::get_filtered_role_bindings))
        .route("/getFilteredClusterRoleBindings", post(handlers::get_filtered_cluster_role_bindings))
        .route("/createServiceAccount", post(handlers::create_service_account))
        .route("/createSecret", post(handlers::create_secret))
        .route("/createRoleBinding", post(handlers::create_role_binding))
        .route("/createClusterRoleBinding", post(handlers::create_cluster_role_binding))
        .route("/generateK8sConfig", post(handlers::generate_k8s_config))
        .route("/generateK8sConfigDownloadFile", post(handlers::generate_k8s_config_download))
        .route("/deleteSecret", delete(handlers::delete_secret))
        .route("/deleteServiceAccount", delete(handlers::delete_service_account))
        .route("/deleteRoleBinding", delete(handlers::delete_role_binding))
        .route("/deleteClusterRoleBinding", delete(handlers::delete_cluster_role_binding))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth::auth_middleware));

    Router::new()
        .route("/apps/login", post(handlers::login))
        .nest("/apps", protected_routes)
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                })
                .on_request(|request: &axum::http::Request<_>, _span: &tracing::Span| {
                    tracing::info!("started {} {}", request.method(), request.uri().path());
                })
                .on_response(|response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                    let status = response.status();
                    if status.is_server_error() {
                        tracing::error!(status = %status.as_u16(), latency = ?latency, "finished with server error");
                    } else if status.is_client_error() {
                        tracing::warn!(status = %status.as_u16(), latency = ?latency, "finished with client error");
                    } else {
                        tracing::info!(status = %status.as_u16(), latency = ?latency, "finished successfully");
                    }
                })
        )
        .with_state(state)
}
