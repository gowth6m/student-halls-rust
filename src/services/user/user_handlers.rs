use std::sync::Arc;

use axum::{ response::IntoResponse, Extension, Json };
use serde_json::json;

use crate::{
    api::response::ApiResponse,
    db::MongoConnection,
    models::user_models::{ NewUser, User },
};

/// Handler to get the current user
///
/// ### Parameters
/// - `mongo_conn` - MongoDB connection
///
/// ### Returns
/// - `ApiResponse` with a success message
/// - `ApiResponse` with an error message if the user retrieval fails
///
pub async fn get_current_user(Extension(
    mongo_conn,
): Extension<Arc<MongoConnection>>) -> impl IntoResponse {}

/// Handler to create a new user
///
/// ### Parameters
/// - `mongo_conn` - MongoDB connection
///
/// ### Returns
/// - `ApiResponse` with a success message
/// - `ApiResponse` with an error message if the user creation fails
///
pub async fn create_user(
    Extension(mongo_conn): Extension<Arc<MongoConnection>>,
    Json(payload): Json<NewUser>
) -> impl IntoResponse {
    let users_collection = mongo_conn.collections.user.clone();

    match users_collection.insert_one(payload.to_user(), None).await {
        Ok(result) => {
            return ApiResponse::new(
                201,
                "User created successfully".to_string(),
                json!({ "user_id": result.inserted_id })
            );
        }
        Err(e) => {
            return ApiResponse::new(
                500,
                "Failed to create user".to_string(),
                json!({ "error": e.to_string() })
            );
        }
    }
}
