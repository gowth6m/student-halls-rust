use axum::{ extract::Extension, Json };
use serde_json::{ json, Value };
use std::sync::Arc;
use futures_util::stream::StreamExt;
use crate::{ api::response::ApiResponse, db::MongoConnection };
use crate::models::university_models::University;

/// Handler to get all universities
///
/// ### Parameters
/// - `mongo_conn` - MongoDB connection
///
/// ### Returns
/// - `ApiResponse` with a success message and a list of all universities
/// - `ApiResponse` with an error message if the university retrieval fails
///
pub async fn get_all_universities(Extension(
    mongo_conn,
): Extension<Arc<MongoConnection>>) -> Json<ApiResponse<Value>> {
    let universities_collection = &mongo_conn.collections.university;
    let mut cursor = universities_collection
        .find(None, None).await
        .expect("Failed to execute find");
    let mut universities: Vec<University> = vec![];

    while let Some(result) = cursor.next().await {
        match result {
            Ok(university) => universities.push(university),
            Err(e) => eprintln!("Failed to deserialize document: {}", e),
        }
    }

    let response = ApiResponse::new(
        200,
        "Success".to_string(),
        json!({"universities": universities})
    );

    Json(response)
}

/// Handler to create a new university
///
/// ### Parameters
/// - `mongo_conn` - MongoDB connection
/// - `payload` - University payload
///
/// ### Returns
/// - `ApiResponse` with a success message and the ID of the created university
/// - `ApiResponse` with an error message if the university creation fails
///
pub async fn create_university_handler(
    Extension(mongo_conn): Extension<Arc<MongoConnection>>,
    Json(payload): Json<University>
) -> Json<ApiResponse<Value>> {
    let universities_collection = &mongo_conn.collections.university;

    let result = universities_collection.insert_one(payload, None).await;

    match result {
        Ok(insert_result) => {
            if let Some(oid) = insert_result.inserted_id.as_object_id() {
                Json(
                    ApiResponse::new(
                        201,
                        "University created successfully".to_string(),
                        json!({"id": oid.to_hex()})
                    )
                )
            } else {
                Json(ApiResponse::new(500, "Failed to create university".to_string(), json!({})))
            }
        }
        Err(_) => Json(ApiResponse::new(500, "Database operation failed".to_string(), json!({}))),
    }
}
