use axum::{ Router, routing::get, Json };
use serde_json::{ json, Value };

use crate::api::response::ApiResponse;

pub fn root_routes() -> Router {
    Router::new().route("/", get(root_fn))
}

async fn root_fn() -> Json<ApiResponse<Value>> {
    let response = ApiResponse::new(
        200,
        "Success".to_string(),
        json!({"greeting": "Hello, World!"})
    );
    Json(response)
}
