# Clipstream Backend API (Axum)

A Rust Axum backend API for the Clipstream gaming clip platform with integrated SQLx connection pooling, intelligent video discovery and MinIO processing pipeline.

## 🏗️ Architecture & Deployment

### Subdomain Architecture
- **API Domain**: `api.clipsstream.com` - Dedicated subdomain for all API endpoints
- **Web Domain**: `clipsstream.com` - Main website for the Next.js application
- **Clean Separation**: No path-based routing needed, each service has its own domain
- **Port Configuration**: API runs on port 8000, web app on port 3000

### Service Architecture
- **Axum API Server**: Runs on port 8000, accessible via `api.clipsstream.com`
- **Next.js Frontend**: Runs on port 3000, accessible via `clipsstream.com`  
- **nginx Reverse Proxy**: Routes subdomains to appropriate services
- **PostgreSQL**: Database with connection pooling
- **MinIO**: S3-compatible object storage for video files

### nginx Configuration
```nginx
# API subdomain
server {
    server_name api.clipsstream.com;
    location / {
        proxy_pass http://127.0.0.1:8000;
    }
}

# Main website
server {
    server_name clipsstream.com;
    location / {
        proxy_pass http://127.0.0.1:3000;
    }
}
```

### Docker Compose Setup
```yaml
services:
  clipstream-api:      # Axum server on port 8000
    build: .
    ports: ["8000:8000"]
    
  postgres:            # Database
    image: postgres:15
    ports: ["5432:5432"]
    
  nginx:              # Reverse proxy for subdomains
    image: nginx:alpine
    ports: ["80:80", "443:443"]
```

### Axum + SQLx Integration
- **Framework**: Axum web framework for high-performance async HTTP
- **Database**: SQLx with built-in connection pooling for PostgreSQL
- **Storage**: MinIO S3-compatible object storage with presigned URLs
- **Authentication**: JWT-based stateless authentication
- **Background Jobs**: Redis-based job queue for video processing

### Built-in Connection Pooling Strategy
Each Axum instance maintains its own SQLx connection pool for optimal scalability:

```rust
// Integrated SQLx pooling in main application
let pool = PgPoolOptions::new()
    .max_connections(20)              // Optimal for production load
    .min_connections(5)               // Always-ready connections
    .acquire_timeout(Duration::from_secs(8))
    .idle_timeout(Duration::from_secs(600))
    .max_lifetime(Duration::from_secs(3600))
    .connect(&database_url)
    .await?;

let app = Router::new()
    .route("/auth/verify", post(auth::verify_token))
    .route("/streams", get(streams::list))
    .route("/videos/:id", get(videos::get))
    .with_state(pool); // Shared across all routes
```

### Scaling Benefits
- **Linear Horizontal Scaling**: Add Axum instances behind load balancer
- **No Central Bottleneck**: Each instance manages its own connections
- **Fault Tolerance**: Instance failure doesn't affect other pools
- **Performance**: Direct database access without additional hops

## 🚀 API Endpoints

**Base URL**: `https://api.clipsstream.com`

All endpoints are accessed directly without any path prefix:

### Health & System
```
GET  /health                       # Health check with database status
GET  /                             # Simple hello world endpoint
```

### Authentication & Authorization
```
POST /auth/verify                  # Verify Google ID token and create/update user
GET  /auth/user                    # Get current user info (requires Bearer token)
POST /auth/refresh                 # Refresh app JWT token
```

**Authentication Flow:**
1. User signs in with Google on Next.js frontend
2. Frontend receives Google ID token
3. Frontend sends Google ID token to `https://api.clipsstream.com/auth/verify`
4. Backend verifies token with Google's public keys
5. Backend returns app JWT for subsequent API calls

**Responses:**
- `POST /auth/verify`
  - `200`: `{ user: { id, email, name, avatar }, app_token: "jwt_token", expires_at: "2025-08-08T18:30:00Z" }`
  - `401`: `{ error: "Invalid Google ID token" }`
  - `500`: `{ error: "Failed to verify token with Google" }`

- `GET /auth/user`
  - `200`: `{ id, email, name, avatar, created_at, total_videos, total_likes }`
  - `401`: `{ error: "Invalid or expired token" }`

- `POST /auth/refresh`
  - `200`: `{ app_token: "new_jwt_token", expires_at: "2025-08-08T18:30:00Z" }`
  - `401`: `{ error: "Invalid refresh token" }`

### Stream Management
```
GET    /streams                    # List user's streams
POST   /streams                    # Create new stream
GET    /streams/{id}               # Get stream details
PUT    /streams/{id}               # Update stream settings
DELETE /streams/{id}               # Delete stream

GET    /streams/{id}/members       # List stream members
PUT    /streams/{id}/members/{user_id} # Update member role
DELETE /streams/{id}/members/{user_id} # Remove member

GET    /streams/{id}/invites       # List active invite links (Admin only)
POST   /streams/{id}/invites       # Create new invite link (Admin only)
GET    /streams/{id}/invites/{code} # Get invite details (Admin only)
PUT    /streams/{id}/invites/{code} # Update invite settings (Admin only)
DELETE /streams/{id}/invites/{code} # Revoke invite link (Admin only)

POST   /invites/{code}/join        # Join stream using invite code
GET    /invites/{code}             # Get public invite information
```

**Responses:**
- `GET /streams`
  - `200`: `{ streams: [{ id, name, description, role, member_count, video_count, created_at }] }`
  - `401`: `{ error: "Authentication required" }`

- `POST /streams`
  - `201`: `{ id, name, description, owner_id, created_at }`
  - `400`: `{ error: "Invalid stream data", details: ["name is required"] }`
  - `401`: `{ error: "Authentication required" }`

- `GET /streams/{id}`
  - `200`: `{ id, name, description, owner, settings, user_role, member_count }`
  - `403`: `{ error: "Access denied" }`
  - `404`: `{ error: "Stream not found" }`

- `POST /streams/{id}/invites`
  - `201`: `{ code, role, expires_at, usage_limit, created_at }`
  - `400`: `{ error: "Invalid invite parameters", details: ["role must be creator or viewer"] }`
  - `403`: `{ error: "Admin access required" }`

- `POST /invites/{code}/join`
  - `200`: `{ stream_id, role, joined_at }`
  - `400`: `{ error: "Invite expired" }`
  - `400`: `{ error: "Usage limit exceeded" }`
  - `404`: `{ error: "Invalid invite code" }`
  - `409`: `{ error: "Already a member of this stream" }`

### Video Management & Upload
```
GET    /streams/{id}/videos        # List videos in stream
POST   /streams/{id}/videos        # Upload video (triggers processing pipeline)
GET    /videos/{id}                # Get video details
PUT    /videos/{id}                # Update video metadata
DELETE /videos/{id}                # Delete video

GET    /videos/{id}/stream         # Get video stream URLs (HLS/DASH)
POST   /videos/{id}/upload-url     # Get presigned upload URL for large files
GET    /videos/{id}/processing     # Get processing status
```

**Responses:**
- `GET /streams/{id}/videos`
  - `200`: `{ videos: [{ id, title, duration, processing_index, uploaded_at, uploaded_by, thumbnail_url }] }`
  - `403`: `{ error: "Access denied" }`
  - `404`: `{ error: "Stream not found" }`

- `POST /streams/{id}/videos`
  - `201`: `{ id, processing_index: 1, upload_url: "presigned_url" }` (for large files)
  - `201`: `{ id, processing_index: 1 }` (for direct upload)
  - `400`: `{ error: "Invalid video file", details: ["file must be .mp4, .avi, or .mov"] }`
  - `403`: `{ error: "Creator access required" }`
  - `413`: `{ error: "File too large", max_size: "2GB" }`

- `GET /videos/{id}/stream`
  - `200`: `{ hls_url: "stream_url", dash_url: "stream_url", bitrates: [480, 720, 1080] }` (only if processing_index = 0)
  - `202`: `{ error: "Video still processing", processing_index: 2, estimated_time: "5 minutes" }`
  - `403`: `{ error: "Access denied" }`
  - `404`: `{ error: "Video not found" }`

### Video Processing & Intelligence
```
GET  /videos/{id}/duplicates       # Get duplicate info (if processing_index = -1)
GET  /videos/{id}/similar          # Get similar clips (only if complete)
GET  /videos/{id}/trimmed          # Get trimmed clips with timeline data
GET  /videos/{id}/pov              # Get different POV clips
GET  /videos/{id}/transcript       # Get speech-to-text data
GET  /videos/{id}/embeddings       # Get video embeddings
GET  /videos/{id}/timeline         # Get timeline view data

GET  /processing/queue             # Get processing queue status (Admin)
GET  /processing/stats             # Get processing statistics (Admin)
```

**Responses:**
- `GET /videos/{id}/duplicates`
  - `200`: `{ is_duplicate: true, original_video: { id, title, uploaded_by, uploaded_at }, similarity: 1.0 }`
  - `404`: `{ error: "Not a duplicate video" }`

- `GET /videos/{id}/similar`
  - `200`: `{ similar_videos: [{ id, title, similarity: 0.85, thumbnail_url }] }`
  - `202`: `{ error: "Processing not complete" }`

- `GET /videos/{id}/timeline`
  - `200`: `{ original_duration: 300, trimmed_clips: [{ id, title, start_time: 45, duration: 30, thumbnail_url }] }`
  - `202`: `{ error: "Processing not complete" }`

- `GET /processing/queue`
  - `200`: `{ queue_length: 5, current_job: { video_id: "123", stage: "transcoding" }, estimated_wait: "10 minutes" }`
  - `403`: `{ error: "Admin access required" }`

### Search & Discovery
```
GET  /search                       # Search videos by content/speech
GET  /search/suggestions           # Get search suggestions
```

**Responses:**
- `GET /search`
  - `200`: `{ videos: [{ id, title, snippet: "Nice shot!", timestamp: 12.5, thumbnail_url }], total_results: 47, page: 1 }`
  - `400`: `{ error: "Search query required" }`

### Social Features
```
POST /videos/{id}/like             # Like/unlike video
GET  /videos/{id}/likes            # Get like count and user's like status
POST /videos/{id}/share            # Generate share link
GET  /videos/{id}/shares           # Get share count
```

**Responses:**
- `POST /videos/{id}/like`
  - `200`: `{ liked: true, total_likes: 15 }`
  - `200`: `{ liked: false, total_likes: 14 }`

- `POST /videos/{id}/share`
  - `201`: `{ share_url: "https://api.clipsstream.com/share/abc123", expires_at: "2025-08-15T10:30:00Z" }`

### File Serving
```
GET  /files/videos/{id}/thumbnail  # Get video thumbnail
GET  /files/videos/{id}/stream     # Get video file stream
GET  /share/{code}                 # Access shared video
```

**Responses:**
- `GET /files/videos/{id}/thumbnail`
  - `200`: Returns image file (JPEG/PNG)
  - `200`: Returns template with processing overlay (if processing_index = 1)
  - `200`: Returns grayed thumbnail with "duplicate" overlay (if processing_index = -1)
  - `404`: `{ error: "Thumbnail not found" }`

### System & Admin
```
GET  /health                       # Health check (also available at root /)
GET  /system/storage               # Storage usage stats (Admin)
GET  /admin/duplicates             # List flagged duplicates (Admin)
POST /admin/processing/retry       # Retry failed processing jobs (Admin)
```

**Responses:**
- `GET /health`
  - `200`: `{ status: "healthy", database: "connected", storage: "available", ffmpeg: "ready", uptime: "5 days" }`
  - `503`: `{ status: "unhealthy", issues: ["database_connection_failed"] }`

- `GET /system/storage`
  - `200`: `{ total_space: "500GB", used_space: "230GB", free_space: "270GB", video_count: 1247 }`
  - `403`: `{ error: "Admin access required" }`

## 🔄 Video Processing Pipeline with MinIO

Automatic 5-stage processing workflow integrated with MinIO object storage:

### MinIO Integration Flow
1. **Upload**: Frontend uploads directly to MinIO via presigned URLs
2. **Webhook Trigger**: MinIO notifies API of completed uploads  
3. **Processing**: Workers download from MinIO, process, and upload results back
4. **Streaming**: Users stream processed videos directly from MinIO

### Processing Stages with Storage
- **Stage 1 - Duplicate Detection**:
  - Downloads metadata from `raw-uploads/{video_id}.mp4`
  - Compares file hashes without full download
  - Updates database status

- **Stage 2 - Video Transcoding**:
  - Downloads raw video from MinIO
  - Transcodes to multiple bitrates locally
  - Uploads HLS segments to `processed-videos/{video_id}/`
  - Generates thumbnails to `thumbnails/{video_id}.jpg`

- **Stage 3 - Speech-to-Text**:
  - Extracts audio from MinIO-stored video
  - Processes with speech recognition
  - Saves transcript to `transcripts/{video_id}.json`

- **Stage 4 - AI Analysis**:
  - Downloads processed video for embedding generation
  - Runs similarity analysis and POV detection
  - Stores embeddings in `embeddings/{video_id}.json`

### Storage Status Codes
- `1`: Duplicate detection (accessing `raw-uploads/`)
- `2`: Video transcoding (creating `processed-videos/`)
- `3`: Speech-to-text (creating `transcripts/`)
- `4`: AI analysis (creating `embeddings/`)
- `0`: Processing complete (all assets in MinIO)
- `-1`: Flagged as duplicate

### Background Processing Architecture
- **Upload Handler**: Generates presigned MinIO URLs using shared SQLx pool
- **Worker Pool**: Multiple Tokio tasks with MinIO S3 clients and database access
- **Queue System**: Redis-based job queue with MinIO object keys
- **Progress Tracking**: Real-time status updates via SQLx pool connections
- **Connection Management**: Background workers share the same SQLx pool for efficiency

```rust
// Background processing with shared SQLx pool
async fn process_video_worker(pool: PgPool, redis: RedisPool) {
    while let Some(job) = redis.dequeue("video_processing").await {
        // Use shared pool for database operations
        let video = sqlx::query_as!(
            Video,
            "SELECT * FROM videos WHERE id = $1",
            job.video_id
        )
        .fetch_one(&pool)
        .await?;

        // Process with MinIO, update status via pool
        process_video_stage(&video, &pool).await?;
    }
}
```

### Connection Pool Monitoring
```rust
// Health check endpoint showing pool status
async fn health_check(State(pool): State<PgPool>) -> Json<HealthStatus> {
    Json(HealthStatus {
        database: "healthy",
        pool_size: pool.size(),
        pool_idle: pool.num_idle(),
        pool_connections: pool.size() - pool.num_idle(),
    })
}
```
- **Progress Tracking**: Real-time status updates via WebSocket or polling

## 🛠️ Tech Stack

- **Framework**: Axum (async Rust web framework)
- **Database**: PostgreSQL with SQLx
- **File Storage**: S3-compatible storage (MinIO/AWS S3)
- **Queue**: Redis with background workers
- **Video Processing**: FFmpeg bindings
- **Authentication**: JWT with Google token verification
- **AI/ML**: ONNX Runtime for embeddings, custom DTW implementation

## 📊 Production Features

### Scalability
- **Horizontal scaling**: Multiple Axum instances behind load balancer
- **Worker scaling**: Independent processing workers
- **Database pooling**: Connection pooling with SQLx
- **Caching**: Redis for session data and frequently accessed content

### Monitoring & Observability
- **Health checks**: Detailed system status endpoints
- **Metrics**: Prometheus metrics export
- **Logging**: Structured logging with tracing
- **Error handling**: Comprehensive error types with detailed context

### Security
- **JWT validation**: Google ID token verification + app JWT
- **Rate limiting**: Per-user and per-endpoint limits
- **Input validation**: Strict type checking with serde
- **CORS**: Configurable cross-origin policies
