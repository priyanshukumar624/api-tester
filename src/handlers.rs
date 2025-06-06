use actix_web::{web, HttpResponse, Responder};
use crate::models::{ApiRequest, ApiResponse};
use reqwest;
use std::collections::HashMap;

pub async fn test_api(req: web::Json<ApiRequest>) -> impl Responder {
     println!("📥 Request received!");
    let client = reqwest::Client::new();
    let mut request_builder = match req.method.to_uppercase().as_str() {
        "GET" => client.get(&req.url),
        "POST" => client.post(&req.url),
        "PUT" => client.put(&req.url),
        "DELETE" => client.delete(&req.url),
        _ => return HttpResponse::BadRequest().body("Unsupported HTTP method"),
    };

    if let Some(headers) = &req.headers {
        for (key, value) in headers {
            request_builder = request_builder.header(key, value);
        }
    }

    if let Some(body) = &req.body {
        request_builder = request_builder.body(body.clone());
    }

    match request_builder.send().await {
        Ok(response) => {
            let status = response.status().as_u16();
            let headers = response.headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect::<HashMap<String, String>>();
            let body = response.text().await.unwrap_or_else(|_| "Error reading body".to_string());

            HttpResponse::Ok().json(ApiResponse { status, headers, body })
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Request error: {}", e))
        }
    }
}
