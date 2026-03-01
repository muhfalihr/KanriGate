use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct MetaData {
    pub status: u16,
    pub message: String,
    pub exec_time: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub pagination: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[aliases(
    KanriGateRespString = KanriGateResp<String>,
    KanriGateRespVecString = KanriGateResp<Vec<String>>,
)]
pub struct KanriGateResp<T> {
    pub meta_data: MetaData,
    pub data: T,
}

impl<T> KanriGateResp<T> {
    pub fn new(status: u16, message: impl Into<String>, exec_time: f64, data: T) -> Self {
        Self {
            meta_data: MetaData {
                status,
                message: message.into(),
                exec_time,
                pagination: None,
            },
            data,
        }
    }
}
