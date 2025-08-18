use axum::{
    extract::Path,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct InviteResponse {
    message: String,
    purpose: String,
}

async fn join_stream(Path(code): Path<String>) -> Json<InviteResponse> {
    Json(InviteResponse {
        message: format!("Join stream with invite code {} endpoint", code),
        purpose: "Join stream using invite code, validates expiration and usage limits".to_string(),
    })
}

async fn get_invite_info(Path(code): Path<String>) -> Json<InviteResponse> {
    Json(InviteResponse {
        message: format!("Get invite info for code {} endpoint", code),
        purpose: "Get public invite information including stream name and role without joining".to_string(),
    })
}

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/invites/:code/join", post(join_stream))
        .route("/invites/:code", get(get_invite_info))
}
