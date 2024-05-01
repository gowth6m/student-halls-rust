use axum::{ Router, routing::{ get, post }, middleware as axum_middleware };
use crate::services::user::user_handlers::{ create_user, get_current_user };
use crate::middleware::auth_guard::auth_guard;

pub fn user_routes() -> Router {
    Router::new()
        .route("/create", post(create_user))
        // .route("/login", post(todo!()))
        .route("/current", get(get_current_user).layer(axum_middleware::from_fn(auth_guard)))
}
