use std::sync::Arc;

use axum::{ Extension, Router };
use db::MongoConnection;

pub mod api;
pub mod routes;
pub mod services;
pub mod db;
pub mod models;
pub mod middleware;

pub async fn setup_server() -> Router {
    let mongo_conn = MongoConnection::new().await.expect("Failed to initialize MongoDB");
    let shared_mongo_conn = Arc::new(mongo_conn);

    Router::new()
        .nest("/", Router::new().merge(routes::root_routes::root_routes()))
        .nest("/user", Router::new().merge(routes::user_routes::user_routes()))
        .nest("/hall", Router::new().merge(routes::hall_routes::hall_routes()))
        .nest("/university", Router::new().merge(routes::university_routes::university_routes()))
        .layer(Extension(shared_mongo_conn))
}
