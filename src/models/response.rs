use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    pub status: u16,
    pub message: String,
    pub exec_time: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
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
