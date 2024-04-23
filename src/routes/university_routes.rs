use axum::{ routing::{ get, post }, Router };

use crate::services::university::university_handlers::{
    create_university_handler,
    get_all_universities,
};

pub fn university_routes() -> Router {
    return Router::new()
        .route("/all", get(get_all_universities))
        .route("/create", post(create_university_handler));
}
