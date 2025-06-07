use std::collections::HashMap;
use serde::{Deserialize, Serialize};
 use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct ApiRequest {
    pub method: String,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Value>,  // Change here to accept JSON value
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}
