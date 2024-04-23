use axum::{ extract::Extension, Json };
use serde_json::{ json, Value };
use std::sync::Arc;
use crate::{ api::response::ApiResponse, db::MongoConnection };
use mongodb::Collection;
use futures_util::stream::StreamExt;

use crate::models::university_models::University;

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

pub async fn create_university_handler(
    Extension(mongo_conn): Extension<Arc<MongoConnection>>,
    Json(payload): Json<University>
) -> Json<ApiResponse<Value>> {
    let universities_collection: &Collection<University> = &mongo_conn.collections.university;

    match universities_collection.insert_one(payload, None).await {
        Ok(insert_result) => {
            if let Some(oid) = insert_result.inserted_id.as_object_id() {
                let response = ApiResponse::new(
                    201,
                    "University created successfully".to_string(),
                    json!({"id": oid.to_hex()})
                );
                Json(response)
            } else {
                let response = ApiResponse::new(
                    500,
                    "Failed to create university".to_string(),
                    json!({})
                );
                Json(response)
            }
        }
        Err(_) => {
            let response = ApiResponse::new(
                500,
                "Database operation failed".to_string(),
                json!({})
            );
            Json(response)
        }
    }
}
