mod config;
mod models;
mod state;
mod api;
mod services;

use std::sync::Arc;
use kube::{Client, Config};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use clap::Parser;
use config::BaseConfig;
use state::AppState;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Parser)]
struct KubeConfigArgs {
    #[arg(short, long, env = "KUBECONFIG")]
    kube_config: Option<String>,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        api::handlers::login,
        api::handlers::get_templates,
        api::handlers::get_namespaces,
        api::handlers::get_service_accounts,
        api::handlers::create_service_account,
        api::handlers::delete_service_account,
        api::handlers::create_secret,
        api::handlers::delete_secret,
        api::handlers::create_role_binding,
        api::handlers::delete_role_binding,
        api::handlers::create_cluster_role_binding,
        api::handlers::delete_cluster_role_binding,
        api::handlers::get_filtered_role_bindings,
        api::handlers::get_filtered_cluster_role_bindings,
        api::handlers::generate_k8s_config,
        api::handlers::generate_k8s_config_download,
    ),
    components(
        schemas(
            models::MetaData,
            models::UsernameQuery,
            models::RoleBindingQuery,
            models::ClusterRoleBindingQuery,
            models::K8sConfigQuery,
            models::LoginRequest,
            models::AuthBody,
            models::KanriGateRespString,
            models::KanriGateRespVecString,
        )
    ),
    tags(
        (name = "KanriGate API", description = "**Kanrigate** is a professional, high-performance Kubernetes RBAC (Role-Based Access Control) Management tool. It combines a modern **SvelteKit** frontend with a blazing-fast **Rust** backend to provide a seamless experience for managing cluster permissions.")
    )
)]
pub struct ApiDoc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let args = KubeConfigArgs::parse();
    
    let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "kanrigate=info,tower_http=info".into());

    if env == "production" {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().json().with_current_span(true))
            .with(filter)
            .init();
    } else {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().with_thread_ids(true))
            .with(filter)
            .init();
    }

    tracing::info!(env = %env, "KanriGate backend initializing...");
    let config = Arc::new(BaseConfig::new().expect("Failed to load configuration"));

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        run(config, args.kube_config).await
    })
}

async fn run(config: Arc<BaseConfig>, kube_config_path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Initializing Kubernetes client...");
    
    let k8s_config = if let Some(path) = kube_config_path {
        let kubeconfig = kube::config::Kubeconfig::read_from(path)?;
        Config::from_custom_kubeconfig(kubeconfig, &Default::default()).await?
    } else {
        Config::infer().await?
    };
    
    let client = Client::try_from(k8s_config)?;

    let state = AppState {
        client,
        config: config.clone(),
    };

    let app = api::routes::app_router(state)
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()));

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("Starting KanriGate API server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
