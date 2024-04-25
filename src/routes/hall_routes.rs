use axum::{ routing::{ get, post }, Router };

use crate::services::hall::hall_handlers::{ create_hall_handler, get_all_halls };

pub fn hall_routes() -> Router {
    return Router::new()
        .route("/all", get(get_all_halls))
        .route("/create", post(create_hall_handler));
}
