use config::Config;
use serde::Deserialize;
use validator::{Validate, ValidationError};
use reqwest::blocking::Client;
use std::time::Duration;
use regex::Regex;
use lazy_static::lazy_static;

pub struct Constants;

pub mod rbac {
    pub const API_VERSION: &str = "rbac.authorization.k8s.io/v1";
    pub const API_GROUP: &str = "rbac.authorization.k8s.io";
}

pub mod sa {
    pub const ANNOTATIONS_NAME: &str = "kubernetes.io/service-account.name";
    pub const TYPE_TOKEN: &str = "kubernetes.io/service-account-token";
}

pub mod permissions {
    pub const TEMPLATES: [&str; 3] = ["operation", "monitoring", "developer"];
    pub const CLUSTER: [&str; 3] = ["admin", "read-only", "none"];
}

lazy_static! {
    static ref RE_CLUSTER_NAME: Regex = Regex::new(r"^[a-zA-Z0-9-@]+$").unwrap();
}

fn validate_cluster_name(name: &str) -> Result<(), ValidationError> {
    if !RE_CLUSTER_NAME.is_match(name) {
        let mut error = ValidationError::new("invalid_cluster_name");
        error.message = Some(std::borrow::Cow::from("Cluster name contains invalid characters"));
        return Err(error);
    }
    Ok(())
}

fn validate_control_plane_address(url: &str) -> Result<(), ValidationError> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        let mut error = ValidationError::new("invalid_protocol");
        error.message = Some(std::borrow::Cow::from("Control plane address must start with http:// or https://"));
        return Err(error);
    }

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|_| {
            let mut error = ValidationError::new("client_build_error");
            error.message = Some(std::borrow::Cow::from("Failed to build HTTP client"));
            error
        })?;

    let response = client.get(url).send();

    match response {
        Ok(resp) => {
            if resp.status().as_u16() >= 500 {
                let mut error = ValidationError::new("server_error");
                error.message = Some(std::borrow::Cow::from("Control plane not responding properly"));
                return Err(error);
            }
        }
        Err(_) => {
            let mut error = ValidationError::new("connection_error");
            error.message = Some(std::borrow::Cow::from("Unable to connect to control plane"));
            return Err(error);
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct BaseConfig {
    #[validate(range(min=1024, max=65535, message="Port must be between 1024 and 65535"))]
    pub port: u16,

    #[validate(custom(function="validate_cluster_name"))]
    pub cluster_name: String,

    #[validate(custom(function="validate_control_plane_address"))]
    pub control_plane_address: String,

    pub admin_username: String,
    pub admin_password_hash: String,
    pub jwt_secret: String,
}

impl BaseConfig {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let s = Config::builder()
            .set_default("port", 3232)?
            .set_default("cluster_name", "kubernetes-admin@kubernetes")?
            .set_default("control_plane_address", "https://172.17.0.3:6443")?
            .set_default("admin_username", "admin")?
            .set_default("admin_password_hash", "$argon2id$v=19$m=19456,t=2,p=1$Z3YxeXJ3emx6cWZ6Z3YxeXJ3emx6cWZ6$R0p0U0p0U0p0U0p0U0p0U0p0U0p0U0p0U0p0U0p0U0p0")? // default 'admin' password hash
            .set_default("jwt_secret", "replace-with-a-secure-secret-key")?
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
            )
            .build()?;

        let config: BaseConfig = s.try_deserialize()?;
        
        config.validate()?;

        Ok(config)
    }
}