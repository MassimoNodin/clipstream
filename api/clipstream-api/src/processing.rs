use axum::{
    extract::Path,
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct ProcessingResponse {
    message: String,
    purpose: String,
}

async fn get_queue_status() -> Json<ProcessingResponse> {
    Json(ProcessingResponse {
        message: "Processing queue status endpoint".to_string(),
        purpose: "Get current processing queue length, active jobs, and estimated wait times (Admin only)".to_string(),
    })
}

async fn get_processing_stats() -> Json<ProcessingResponse> {
    Json(ProcessingResponse {
        message: "Processing statistics endpoint".to_string(),
        purpose: "Get detailed processing statistics, success rates, and performance metrics (Admin only)".to_string(),
    })
}

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/processing/queue", get(get_queue_status))
        .route("/processing/stats", get(get_processing_stats))
}
