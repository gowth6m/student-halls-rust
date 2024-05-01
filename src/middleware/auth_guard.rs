use std::env;
use axum::{
    http::{ Request, StatusCode },
    middleware::Next,
    response::IntoResponse,
    headers::{ HeaderMapExt, Authorization, authorization::Bearer },
};
use jsonwebtoken::{ DecodingKey, Validation, Algorithm, decode, errors::ErrorKind };
use once_cell::sync::Lazy;
use serde_json::json;
use crate::{ api::response::ApiResponse, models::auth_models::Claims };

static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    env::var("JWT_SECRET").expect("Expected JWT_SECRET to be set in the environment")
});

pub async fn auth_guard<T>(request: Request<T>, next: Next<T>) -> impl IntoResponse
    where T: Send + 'static
{
    let token = match request.headers().typed_get::<Authorization<Bearer>>() {
        Some(Authorization(bearer)) => bearer.token().to_string(),
        None => {
            return ApiResponse::new(
                StatusCode::UNAUTHORIZED.as_u16(),
                "Unauthorized".to_string(),
                json!({
                    "message": "Authorization header is missing"
                })
            ).into_response();
        }
    };

    let result = decode_and_validate_token(&token);

    match result {
        Ok(_) => next.run(request).await,
        Err((status_code, message)) => {
            let message_clone = message.clone();
            ApiResponse::new(
                status_code.as_u16(),
                message,
                json!({
                    "message": message_clone
                })
            ).into_response()
        }
    }
}

// Helper function to decode and validate token
fn decode_and_validate_token(token: &str) -> Result<Claims, (StatusCode, String)> {
    let decoding_key = DecodingKey::from_secret(JWT_SECRET.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(claims) => Ok(claims.claims),
        Err(err) => {
            let error_message = match *err.kind() {
                ErrorKind::InvalidToken => "Invalid token",
                ErrorKind::ExpiredSignature => "Token is expired",
                ErrorKind::InvalidIssuer => "Invalid issuer",
                _ => "Invalid token",
            };
            Err((StatusCode::UNAUTHORIZED, error_message.to_string()))
        }
    }
}
