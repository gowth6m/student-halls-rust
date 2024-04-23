use axum::{ Router, routing::get, Json };
use serde_json::{ json, Value };

use crate::api::response::ApiResponse;

pub fn root_routes() -> Router {
    Router::new().route("/", get(root_fn))
}

async fn root_fn() -> Json<ApiResponse<Value>> {
    let response = ApiResponse::new(
        200,
        "Welcome to Student-halls REST API!".to_string(),
        json!({
                "author": "Gowthaman Ravindrathas",
                "version": "0.1.0",
                "openapi": "/api-docs"
            })
    );
    Json(response)
}
