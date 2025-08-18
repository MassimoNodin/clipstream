use axum::{
    extract::Path,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct StreamResponse {
    message: String,
    purpose: String,
}

async fn list_streams() -> Json<StreamResponse> {
    Json(StreamResponse {
        message: "List streams endpoint".to_string(),
        purpose: "List user's streams with role information, member counts, and video counts".to_string(),
    })
}

async fn create_stream() -> Json<StreamResponse> {
    Json(StreamResponse {
        message: "Create stream endpoint".to_string(),
        purpose: "Create a new stream with name, description, and initial settings".to_string(),
    })
}

async fn get_stream(Path(id): Path<String>) -> Json<StreamResponse> {
    Json(StreamResponse {
        message: format!("Get stream {} endpoint", id),
        purpose: "Get detailed stream information including settings, owner, and user's role".to_string(),
    })
}

async fn update_stream(Path(id): Path<String>) -> Json<StreamResponse> {
    Json(StreamResponse {
        message: format!("Update stream {} endpoint", id),
        purpose: "Update stream settings, name, description, and other metadata".to_string(),
    })
}

async fn delete_stream(Path(id): Path<String>) -> Json<StreamResponse> {
    Json(StreamResponse {
        message: format!("Delete stream {} endpoint", id),
        purpose: "Delete stream and all associated videos and member data".to_string(),
    })
}

async fn list_members(Path(id): Path<String>) -> Json<StreamResponse> {
    Json(StreamResponse {
        message: format!("List members of stream {} endpoint", id),
        purpose: "List all members of the stream with their roles and join dates".to_string(),
    })
}

async fn update_member(Path((id, user_id)): Path<(String, String)>) -> Json<StreamResponse> {
    Json(StreamResponse {
        message: format!("Update member {} role in stream {} endpoint", user_id, id),
        purpose: "Update member role (creator/viewer) in the stream".to_string(),
    })
}

async fn remove_member(Path((id, user_id)): Path<(String, String)>) -> Json<StreamResponse> {
    Json(StreamResponse {
        message: format!("Remove member {} from stream {} endpoint", user_id, id),
        purpose: "Remove member from stream and revoke access to all videos".to_string(),
    })
}

async fn list_invites(Path(id): Path<String>) -> Json<StreamResponse> {
    Json(StreamResponse {
        message: format!("List invites for stream {} endpoint", id),
        purpose: "List active invite links with codes, roles, and usage statistics (Admin only)".to_string(),
    })
}

async fn create_invite(Path(id): Path<String>) -> Json<StreamResponse> {
    Json(StreamResponse {
        message: format!("Create invite for stream {} endpoint", id),
        purpose: "Create new invite link with role, expiration, and usage limits (Admin only)".to_string(),
    })
}

async fn get_invite(Path((id, code)): Path<(String, String)>) -> Json<StreamResponse> {
    Json(StreamResponse {
        message: format!("Get invite {} details for stream {} endpoint", code, id),
        purpose: "Get detailed invite information including usage statistics (Admin only)".to_string(),
    })
}

async fn update_invite(Path((id, code)): Path<(String, String)>) -> Json<StreamResponse> {
    Json(StreamResponse {
        message: format!("Update invite {} settings for stream {} endpoint", code, id),
        purpose: "Update invite settings like expiration date and usage limits (Admin only)".to_string(),
    })
}

async fn revoke_invite(Path((id, code)): Path<(String, String)>) -> Json<StreamResponse> {
    Json(StreamResponse {
        message: format!("Revoke invite {} for stream {} endpoint", code, id),
        purpose: "Revoke invite link and prevent further usage (Admin only)".to_string(),
    })
}

async fn list_videos(Path(id): Path<String>) -> Json<StreamResponse> {
    Json(StreamResponse {
        message: format!("List videos in stream {} endpoint", id),
        purpose: "List all videos in stream with metadata, processing status, and thumbnails".to_string(),
    })
}

async fn upload_video(Path(id): Path<String>) -> Json<StreamResponse> {
    Json(StreamResponse {
        message: format!("Upload video to stream {} endpoint", id),
        purpose: "Upload video file or get presigned URL for large files, triggers processing pipeline".to_string(),
    })
}

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/streams", get(list_streams))
        .route("/streams", post(create_stream))
        .route("/streams/:id", get(get_stream))
        .route("/streams/:id", put(update_stream))
        .route("/streams/:id", delete(delete_stream))
        .route("/streams/:id/members", get(list_members))
        .route("/streams/:id/members/:user_id", put(update_member))
        .route("/streams/:id/members/:user_id", delete(remove_member))
        .route("/streams/:id/invites", get(list_invites))
        .route("/streams/:id/invites", post(create_invite))
        .route("/streams/:id/invites/:code", get(get_invite))
        .route("/streams/:id/invites/:code", put(update_invite))
        .route("/streams/:id/invites/:code", delete(revoke_invite))
        .route("/streams/:id/videos", get(list_videos))
        .route("/streams/:id/videos", post(upload_video))
}
