# Clipstream Development Roadmap 🎯

This roadmap outlines the development order for Clipstream, from core infrastructure to production features. Track progress with the checkboxes below.

---

<details open>
<summary>Phase 1: Foundation & Core Infrastructure</summary>

### 1.1 Backend Core Setup
- [ ] Basic Axum server with SQLx connection pooling
- [ ] Health check endpoint (`/system/health`)
- [ ] Database connection and migration system
- [ ] Environment variable configuration

#### Database Schema
- [ ] Users table with Google OAuth fields
- [ ] Streams table with ownership/settings
- [ ] Videos table with `processing_index` field
- [ ] Stream_members table with roles
- [ ] Basic indexes and constraints

#### Authentication System
- [ ] JWT token generation/validation
- [ ] Google ID token verification (`POST /auth/verify`)
- [ ] User creation/update logic
- [ ] Authentication middleware for protected routes

### 1.2 MinIO Integration
- [ ] Docker Compose configuration
- [ ] Initial bucket creation (`clipstream` bucket)
- [ ] Basic bucket policies for public thumbnails
- [ ] Health check integration

#### Presigned URL Generation
- [ ] POST `/streams/{id}/videos` - Generate upload URLs
- [ ] MinIO S3 client integration
- [ ] Basic file validation (format, size limits)

</details>

<details>
<summary>Phase 2: Core Video Management</summary>

### 2.1 Basic Video Operations
- [ ] Frontend direct upload to MinIO via presigned URLs
- [ ] Video record creation in database (`processing_index = 1`)
- [ ] Basic error handling and validation
- [ ] GET `/streams/{id}/videos` - List videos in stream
- [ ] GET `/videos/{id}` - Get video details
- [ ] Basic thumbnail serving from MinIO

### 2.2 Stream Management
- [ ] POST `/streams` - Create stream
- [ ] GET `/streams` - List user streams
- [ ] GET `/streams/{id}` - Stream details
- [ ] Stream ownership validation
- [ ] Stream member roles (owner, admin, creator, viewer)
- [ ] Route protection based on roles
- [ ] Basic permission checking middleware

</details>

<details>
<summary>Phase 3: Video Processing Pipeline</summary>

### 3.1 Processing Infrastructure
- [ ] Redis Docker service
- [ ] Job queue structure for video processing
- [ ] Basic worker framework

#### Stage 1: Duplicate Detection
- [ ] File hash calculation without full download
- [ ] Duplicate checking against existing videos
- [ ] Set `processing_index` to -1 for duplicates
- [ ] Duplicate thumbnail overlay generation

### 3.2 Core Processing Stages
#### Stage 2: Video Transcoding
- [ ] FFmpeg integration for video processing
- [ ] Multi-bitrate HLS generation (480p, 720p, 1080p)
- [ ] Upload transcoded files to `MinIO/processed-videos/`
- [ ] Thumbnail generation and upload

#### Processing Status Updates
- [ ] Real-time `processing_index` updates via SQLx pool
- [ ] GET `/videos/{id}/processing` endpoint
- [ ] Frontend polling for processing status

</details>

<details>
<summary>Phase 4: Frontend Development</summary>

### 4.1 Next.js Core Setup
#### Authentication Frontend
- [ ] NextAuth.js configuration with Google provider
- [ ] Authentication state management
- [ ] Protected route components

#### Basic UI Components
- [ ] Stream list and creation forms
- [ ] Video upload component with progress
- [ ] Video list/grid display
- [ ] Processing status indicators

### 4.2 Video Player Integration
- [ ] HLS.js integration for adaptive streaming
- [ ] Direct streaming from MinIO processed-videos/
- [ ] Basic video controls and quality selection

</details>

<details>
<summary>Phase 5: Advanced Processing</summary>

### 5.1 Speech & AI Features
#### Stage 3: Speech-to-Text
- [ ] Audio extraction from video files
- [ ] Speech recognition integration
- [ ] Transcript storage in MinIO transcripts/
- [ ] Basic search by transcript content

#### Stage 4: AI Analysis
- [ ] OpenAI embeddings generation
- [ ] Similarity calculation between videos
- [ ] Embedding storage in MinIO embeddings/
- [ ] Basic similar video recommendations

</details>

<details>
<summary>Phase 6: Community Features</summary>

### 6.1 Invite System
- [ ] POST `/streams/{id}/invites` - Create invite links
- [ ] POST `/invites/{code}/join` - Join via invite
- [ ] Invite expiration and usage limits
- [ ] Frontend invite management UI

### 6.2 Social Features
- [ ] POST `/videos/{id}/like` - Like/unlike system
- [ ] Video sharing with temporary public links
- [ ] Basic video metadata editing

</details>

<details>
<summary>Phase 7: Intelligence Features</summary>

### 7.1 Advanced Discovery
- [ ] GET `/videos/{id}/similar` endpoint
- [ ] AI-powered content recommendations
- [ ] Frontend similar videos display

#### POV Detection
- [ ] Cross-video perspective analysis
- [ ] Timeline alignment algorithms
- [ ] POV cluster visualization

### 7.2 Timeline Features
- [ ] Dynamic Time Warping (DTW) algorithm implementation
- [ ] GET `/videos/{id}/timeline` endpoint
- [ ] Timeline visualization with aligned clips

#### Advanced Search
- [ ] Full-text search across transcripts
- [ ] Content-based video search
- [ ] Search result ranking and filtering

</details>

<details>
<summary>Phase 8: Production Features</summary>

### 8.1 Monitoring & Admin
- [ ] Processing queue monitoring
- [ ] System health metrics
- [ ] User and content management
- [ ] Storage usage statistics

#### Error Handling & Logging
- [ ] Comprehensive error types
- [ ] Structured logging with tracing
- [ ] Processing job retry mechanisms
- [ ] Dead letter queue handling

### 8.2 Performance Optimization
- [ ] Redis caching for frequently accessed data
- [ ] Thumbnail and metadata caching
- [ ] API response caching
- [ ] Load balancer configuration
- [ ] Multiple Axum instance support
- [ ] Database connection pool optimization
- [ ] Worker scaling strategies

</details>

<details>
<summary>Phase 9: Production Deployment</summary>

### 9.1 Infrastructure
- [ ] Multi-stage Docker builds
- [ ] Production environment configuration
- [ ] SSL/TLS certificate management
- [ ] Backup and recovery procedures

### Monitoring & Observability
- [ ] Prometheus metrics collection
- [ ] Health check endpoints
- [ ] Performance monitoring
- [ ] Error tracking and alerting

</details>