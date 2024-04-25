use axum::{ extract::Extension, Json };
use serde_json::{ json, Value };
use std::sync::Arc;
use futures_util::stream::StreamExt;
use crate::{ api::response::ApiResponse, db::MongoConnection };
use crate::models::hall_models::Hall;

/// Handler to get all halls
///
/// ### Parameters
/// - `mongo_conn` - MongoDB connection
///
/// ### Returns
/// - `ApiResponse` with a success message and a list of all halls
/// - `ApiResponse` with an error message if the hall retrieval fails
///
pub async fn get_all_halls(Extension(
    mongo_conn,
): Extension<Arc<MongoConnection>>) -> Json<ApiResponse<Value>> {
    let halls_collection = &mongo_conn.collections.hall;
    let mut cursor = halls_collection.find(None, None).await.expect("Failed to execute find");
    let mut halls: Vec<Hall> = vec![];

    while let Some(result) = cursor.next().await {
        match result {
            Ok(hall) => halls.push(hall),
            Err(e) => eprintln!("Failed to deserialize document: {}", e),
        }
    }

    let response = ApiResponse::new(200, "Success".to_string(), json!({"halls": halls}));

    Json(response)
}

/// Handler to create a new hall
///
/// ### Parameters
/// - `mongo_conn` - MongoDB connection
/// - `payload` - Hall payload
///
/// ### Returns
/// - `ApiResponse` with a success message and the ID of the created hall
/// - `ApiResponse` with an error message if the hall creation fails
///
pub async fn create_hall_handler(
    Extension(mongo_conn): Extension<Arc<MongoConnection>>,
    Json(payload): Json<Hall>
) -> Json<ApiResponse<Value>> {
    let halls_collection = &mongo_conn.collections.hall;

    let result = halls_collection.insert_one(payload, None).await;

    match result {
        Ok(insert_result) => {
            if let Some(oid) = insert_result.inserted_id.as_object_id() {
                Json(
                    ApiResponse::new(
                        201,
                        "Hall created successfully".to_string(),
                        json!({"id": oid.to_hex()})
                    )
                )
            } else {
                Json(ApiResponse::new(500, "Failed to create hall".to_string(), json!({})))
            }
        }
        Err(e) => {
            eprintln!("Failed to create hall: {}", e);
            Json(ApiResponse::new(500, "Failed to create hall".to_string(), json!({})))
        }
    }
}
