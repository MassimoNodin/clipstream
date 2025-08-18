use axum::{
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct SearchResponse {
    message: String,
    purpose: String,
}

async fn search_videos() -> Json<SearchResponse> {
    Json(SearchResponse {
        message: "Search videos endpoint".to_string(),
        purpose: "Search videos by content, speech transcript, and metadata with ranking and snippets".to_string(),
    })
}

async fn search_suggestions() -> Json<SearchResponse> {
    Json(SearchResponse {
        message: "Search suggestions endpoint".to_string(),
        purpose: "Get search term suggestions and autocomplete based on content and popular searches".to_string(),
    })
}

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/search", get(search_videos))
        .route("/search/suggestions", get(search_suggestions))
}
