// SPDX-License-Identifier: GPL-3.0-or-later

use rocket::serde::json::Json;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: bool,
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub answer: serde_json::Value,
    pub used_files: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct QueryRequest {
    pub query: String,
    #[serde(default)]  // makes category optional, defaults to None
    pub category: Option<String>,
}

// Consistent response envelope
#[derive(serde::Serialize)]
pub struct Envelope {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}

impl Envelope {
    pub fn success<D: serde::Serialize>(data: D) -> Self {
        let value = serde_json::to_value(data).expect("Failed to serialize data");
        Self {
            success: true,
            data: Some(value),
            error: None,
        }
    }

    pub fn failure(err: ErrorResponse) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(err),
        }
    }
}

impl From<Envelope> for Json<Value> {
    fn from(envelope: Envelope) -> Self {
        let value = serde_json::to_value(envelope).expect("Envelope serialization failed");
        Json(value)
    }
}