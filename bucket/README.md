# Clipstream MinIO Storage

MinIO object storage configuration for the Clipstream gaming clip platform with automated processing pipeline integration.

## 🗄️ Storage Architecture

MinIO serves as the central file storage hub for all video assets, thumbnails, and processed content. The system uses a bucket-based organization with automated processing triggers.

### Bucket Structure
```
clipstream/
├── raw-uploads/           # Raw video files from user uploads
│   └── {video_id}.mp4     # Original uploaded files
├── processed-videos/      # Transcoded video assets
│   └── {video_id}/
│       ├── 480p.m3u8      # Low quality HLS manifest
│       ├── 720p.m3u8      # Medium quality HLS manifest  
│       ├── 1080p.m3u8     # High quality HLS manifest
│       ├── master.m3u8    # Master HLS playlist
│       └── segments/      # Video segments
├── thumbnails/            # Video thumbnails and overlays
│   ├── {video_id}.jpg     # Final thumbnails
│   ├── {video_id}_processing.jpg  # Processing overlays
│   └── {video_id}_duplicate.jpg   # Duplicate markers
├── transcripts/           # Speech-to-text outputs
│   └── {video_id}.json    # Timestamped transcript data
└── embeddings/            # AI analysis outputs
    └── {video_id}.json    # Video embedding vectors
```

## 🔄 MinIO Processing Pipeline Integration

### Upload Flow with MinIO
```
1. User Upload Request
   ├─> Next.js Frontend requests upload URL
   └─> Axum API generates presigned PUT URL for raw-uploads/

2. Direct MinIO Upload  
   ├─> Frontend uploads file directly to MinIO
   └─> MinIO stores in raw-uploads/{video_id}.mp4

3. Processing Trigger
   ├─> MinIO webhook notifies Axum API of upload completion
   └─> Axum API queues processing job in Redis

4. Processing Stages
   ├─> Workers download from raw-uploads/
   ├─> Process and generate new assets
   ├─> Upload results to respective buckets
   └─> Update database with MinIO paths

### SQLx Connection Pool Integration
Processing workers integrate with Axum's SQLx connection pooling for optimal database performance:
```
```rust
// MinIO operations with shared SQLx pools
async fn generate_presigned_upload_url(
    State(pool): State<PgPool>,           // Shared SQLx connection pool
    State(minio): State<MinioClient>,     // MinIO client
    Json(request): Json<UploadRequest>,
) -> Result<Json<UploadResponse>, StatusCode> {
    // Create video record via connection pool
    let video = sqlx::query!(
        "INSERT INTO videos (id, stream_id, filename, processing_index) VALUES ($1, $2, $3, 1) RETURNING id",
        Uuid::new_v4(),
        request.stream_id,
        request.filename
    )
    .fetch_one(&pool)  // Efficient connection reuse from pool
    .await?;

    // Generate MinIO presigned URL
    let presigned_url = minio.presigned_put_object(
        "raw-uploads",
        &format!("{}.mp4", video.id),
        Duration::from_secs(300)
    ).await?;

    Ok(Json(UploadResponse {
        video_id: video.id,
        presigned_url
    }))
}

// Background processing with shared pools
async fn process_video_worker(
    pool: PgPool,                    // Shared across all workers
    minio: MinioClient,              // MinIO operations
    redis: RedisPool,                // Job queue
) {
    while let Some(job) = redis.dequeue("video_processing").await {
        // Update processing status via shared pool
        sqlx::query!(
            "UPDATE videos SET processing_index = $1 WHERE id = $2",
            job.stage,
            job.video_id
        )
        .execute(&pool)  // Connection managed by pool
        .await?;

        // MinIO download, process, upload cycle
        process_stage(&minio, &job).await?;

        // Final status update via same pool
        sqlx::query!(
            "UPDATE videos SET processing_index = 0, processed_at = NOW() WHERE id = $1",
            job.video_id
        )
        .execute(&pool)
        .await?;
    }
}
```

### Connection Pool Benefits for MinIO Integration
- **Shared Resources**: All MinIO operations use same SQLx pools
- **Transaction Safety**: Database updates coordinated with storage operations
- **Horizontal Scaling**: Each Axum instance manages independent pools
- **Connection Efficiency**: Minimal overhead for status updates and metadata queries

5. Viewing Flow
   ├─> User requests video stream
   ├─> Axum API generates presigned GET URL
   └─> Video player streams directly from MinIO
```

### Processing Worker Integration

#### Stage 1: Duplicate Detection
```rust
// Worker downloads minimal data for hash comparison
let object_metadata = s3_client
    .head_object()
    .bucket("clipstream")
    .key(&format!("raw-uploads/{}.mp4", video_id))
    .send()
    .await?;

// Compare hash without full download
```

#### Stage 2: Video Transcoding  
```rust
// Download raw video for processing
let raw_video = s3_client
    .get_object()
    .bucket("clipstream")  
    .key(&format!("raw-uploads/{}.mp4", video_id))
    .send()
    .await?;

// Process with FFmpeg locally
// Upload transcoded files back to MinIO
s3_client
    .put_object()
    .bucket("clipstream")
    .key(&format!("processed-videos/{}/720p.m3u8", video_id))
    .body(transcoded_content)
    .send()
    .await?;
```

#### Stage 3: Speech-to-Text
```rust
// Extract audio stream and process
// Save transcript to MinIO
s3_client
    .put_object()
    .bucket("clipstream")
    .key(&format!("transcripts/{}.json", video_id))
    .body(transcript_json)
    .content_type("application/json")
    .send()
    .await?;
```

#### Stage 4: AI Analysis
```rust
// Generate embeddings and similarity data  
// Store in MinIO for caching
s3_client
    .put_object()
    .bucket("clipstream")
    .key(&format!("embeddings/{}.json", video_id))
    .body(embeddings_json)
    .send()
    .await?;
```

## 🔧 MinIO Configuration

### Docker Compose Setup
```yaml
version: '3.8'

services:
  minio:
    image: minio/minio:latest
    container_name: clipstream-minio
    ports:
      - "9000:9000"  # API port
      - "9001:9001"  # Console port
    volumes:
      - ./data/minio:/data
    environment:
      - MINIO_ROOT_USER=minioadmin
      - MINIO_ROOT_PASSWORD=minioadmin123
      - MINIO_DOMAIN=minio
    command: server /data --console-address ":9001"
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:9000/minio/health/live"]
      interval: 30s
      timeout: 20s
      retries: 3

  # Create initial bucket and setup webhook
  minio-setup:
    image: minio/mc:latest
    depends_on:
      - minio
    entrypoint: >
      /bin/sh -c "
      sleep 5;
      /usr/bin/mc config host add clipstream-minio http://minio:9000 minioadmin minioadmin123;
      /usr/bin/mc mb clipstream-minio/clipstream --ignore-existing;
      /usr/bin/mc anonymous set download clipstream-minio/clipstream/thumbnails;
      /usr/bin/mc event add clipstream-minio/clipstream arn:minio:sqs::webhook:webhook --event put --prefix raw-uploads/;
      "
```

### Environment Variables
```env
# MinIO Configuration
MINIO_ENDPOINT=http://localhost:9000
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=minioadmin123
MINIO_BUCKET=clipstream
MINIO_REGION=us-east-1
MINIO_USE_SSL=false

# Webhook Configuration  
MINIO_WEBHOOK_URL=http://api:8000/webhooks/minio/upload-complete
```

### Bucket Policies
```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {"AWS": "*"},
      "Action": "s3:GetObject",
      "Resource": "arn:aws:s3:::clipstream/thumbnails/*"
    },
    {
      "Effect": "Allow", 
      "Principal": {"AWS": "*"},
      "Action": "s3:GetObject",
      "Resource": "arn:aws:s3:::clipstream/processed-videos/*"
    }
  ]
}
```

## 🚀 Production Considerations

### Scaling MinIO
```yaml
# Multi-node MinIO cluster for high availability
services:
  minio1:
    image: minio/minio:latest
    volumes:
      - /data1:/data1
      - /data2:/data2
    command: server http://minio{1...4}/data{1...2}
    
  minio2:
    image: minio/minio:latest  
    volumes:
      - /data3:/data1
      - /data4:/data2
    command: server http://minio{1...4}/data{1...2}
```

### Performance Optimization
- **Erasure Coding**: Automatic data protection and recovery
- **Distributed Setup**: Multiple nodes for high availability
- **SSD Storage**: Fast storage for frequent access patterns
- **CDN Integration**: CloudFlare/AWS CloudFront for global distribution

### Monitoring & Observability
```yaml
# Prometheus metrics
minio-prometheus:
  image: prom/prometheus
  command:
    - '--config.file=/etc/prometheus/prometheus.yml'
    - '--storage.tsdb.path=/prometheus'
  volumes:
    - ./prometheus.yml:/etc/prometheus/prometheus.yml
```

### Security Features
- **TLS Encryption**: All data in transit encrypted
- **IAM Policies**: Fine-grained access control
- **Audit Logging**: Complete access audit trail
- **Versioning**: Object versioning for data protection

### Backup Strategy
```bash
# Automated backup to external storage
mc mirror --overwrite minio/clipstream s3/clipstream-backup/

# Point-in-time recovery
mc cp --recursive s3/clipstream-backup/processed-videos/ minio/clipstream/processed-videos/
```

## 🔍 Monitoring & Maintenance

### Health Checks
```bash
# Check MinIO service health
curl http://localhost:9000/minio/health/live

# Check bucket statistics  
mc admin info minio

# Monitor storage usage
mc du minio/clipstream
```

### Performance Metrics
- **Upload/Download Throughput**: Monitor transfer speeds
- **API Response Times**: Track MinIO API performance  
- **Storage Usage**: Monitor bucket size growth
- **Error Rates**: Track failed operations

### Maintenance Tasks
```bash
# Clean up incomplete uploads
mc rm --incomplete --recursive minio/clipstream/

# Verify data integrity
mc admin heal minio/clipstream --recursive

# Update MinIO server
docker-compose pull minio && docker-compose up -d minio
```

This MinIO setup provides a robust, scalable storage foundation that seamlessly integrates with your video processing pipeline while maintaining high performance and reliability.
