use actix_web::{web, HttpResponse, Responder};
use crate::models::models::{ApiRequest, ApiResponse};
use reqwest::header::{HeaderName, HeaderValue};
use serde_json::Value;
use std::collections::HashMap;

pub async fn test_api(req: web::Json<ApiRequest>) -> impl Responder {
    println!("📥 Request received! Method: {}, URL: {}", req.method, req.url);

    let client = reqwest::Client::new();

    // Track if Content-Type header is set explicitly
    let mut content_type_header: Option<HeaderValue> = None;

    // Match method & create request builder
    let mut request_builder = match req.method.to_uppercase().as_str() {
        "GET" => client.get(&req.url),
        "POST" => client.post(&req.url),
        "PUT" => client.put(&req.url),
        "DELETE" => client.delete(&req.url),
        other => {
            return HttpResponse::BadRequest()
                .body(format!("Unsupported HTTP method: {}", other));
        }
    };

    // Add headers carefully & check for Content-Type header
    if let Some(headers) = &req.headers {
        for (key, value) in headers {
            if let (Ok(header_name), Ok(header_value)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_str(value),
            ) {
                if key.to_lowercase() == "content-type" {
                    content_type_header = Some(header_value.clone());
                }
                request_builder = request_builder.header(header_name, header_value);
            } else {
                println!("⚠️ Invalid header skipped: {}: {}", key, value);
            }
        }
    }

    // Add body if present, set default Content-Type if missing
    if let Some(body_value) = &req.body {
        // Body can be string or any JSON value; convert accordingly
        let body_string = match body_value {
            Value::String(s) => s.clone(),
            _ => body_value.to_string(),
        };

        // Set default Content-Type to application/json if not provided
        if content_type_header.is_none() {
            request_builder = request_builder.header("Content-Type", "application/json");
        }

        request_builder = request_builder.body(body_string);
    }

    // Send request to target API
    match request_builder.send().await {
        Ok(response) => {
            let status = response.status().as_u16();

            // Collect response headers
            let headers = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect::<HashMap<String, String>>();

            // Get response body text
            let body = match response.text().await {
                Ok(text) => text,
                Err(_) => "Error reading response body".to_string(),
            };

            // Return JSON response with status, headers, body
            HttpResponse::Ok().json(ApiResponse { status, headers, body })
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Request error: {}", e))
        }
    }
}
