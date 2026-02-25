use kube::Client;
use std::sync::Arc;
use crate::config::BaseConfig;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub config: Arc<BaseConfig>,
}
