use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request payload structure
#[derive(Deserialize, Debug, Clone)]
pub struct RequestPayload {
    pub message: Option<String>,
    pub data: Option<HashMap<String, serde_json::Value>>,
}

/// Response payload structure
#[derive(Serialize, Debug, Clone)]
pub struct ResponsePayload {
    pub status: String,
    pub message: String,
    pub data: Option<HashMap<String, serde_json::Value>>,
    pub timestamp: String,
}
