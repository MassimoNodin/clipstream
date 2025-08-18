use axum::{
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct AuthResponse {
    message: String,
    purpose: String,
}

async fn verify_token() -> Json<AuthResponse> {
    Json(AuthResponse {
        message: "Authentication endpoint".to_string(),
        purpose: "Verify Google ID token and create/update user, returns app JWT for subsequent API calls".to_string(),
    })
}

async fn get_user() -> Json<AuthResponse> {
    Json(AuthResponse {
        message: "User info endpoint".to_string(),
        purpose: "Get current user info including profile data and statistics (requires Bearer token)".to_string(),
    })
}

async fn refresh_token() -> Json<AuthResponse> {
    Json(AuthResponse {
        message: "Token refresh endpoint".to_string(),
        purpose: "Refresh app JWT token to extend authentication session".to_string(),
    })
}

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/auth/verify", post(verify_token))
        .route("/auth/user", get(get_user))
        .route("/auth/refresh", post(refresh_token))
}
