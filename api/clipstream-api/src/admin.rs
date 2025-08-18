use axum::{
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct SystemResponse {
    message: String,
    purpose: String,
}

async fn get_storage_stats() -> Json<SystemResponse> {
    Json(SystemResponse {
        message: "Storage usage statistics endpoint".to_string(),
        purpose: "Get detailed storage usage, space allocation, and video count statistics (Admin only)".to_string(),
    })
}

async fn list_flagged_duplicates() -> Json<SystemResponse> {
    Json(SystemResponse {
        message: "List flagged duplicates endpoint".to_string(),
        purpose: "List all videos flagged as duplicates for admin review and management (Admin only)".to_string(),
    })
}

async fn retry_failed_processing() -> Json<SystemResponse> {
    Json(SystemResponse {
        message: "Retry failed processing jobs endpoint".to_string(),
        purpose: "Retry failed video processing jobs and reset their status for reprocessing (Admin only)".to_string(),
    })
}

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/system/storage", get(get_storage_stats))
        .route("/admin/duplicates", get(list_flagged_duplicates))
        .route("/admin/processing/retry", post(retry_failed_processing))
}
