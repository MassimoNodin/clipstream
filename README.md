# Clipstream 🎮

A self-hosted video platform designed for gaming communities to share, discover, and organize gaming clips with intelligent discovery and MinIO object storage.

## 🌟 Features

### Core Platform
- **Stream-based Organization**: Create private groups (streams) for different games or communities
- **Invite-only Access**: Controlled access with role-based permissions
- **Intelligent Video Discovery**: Find similar clips and different points of view using AI embeddings
- **Multi-bitrate Streaming**: Automatic transcoding for optimal viewing on any connection
- **Speech-to-Text Search**: Find clips by what's said in the video
- **MinIO Object Storage**: Scalable, S3-compatible storage with direct client uploads

### Video Intelligence
- **Similar Clip Detection**: Discover related gameplay moments across your collection
- **POV Recognition**: Find the same moment from different players' perspectives
- **Smart Thumbnails**: Automatically generated thumbnails for quick browsing
- **Embedding-based Recommendations**: AI-powered content suggestions

### Community Features
- **Role Management**: Stream owners, admins, creators, and viewers with distinct permissions
- **Social Interactions**: Like and share your favorite clips
- **Timeline View**: Visual timeline showing all trimmed clips aligned with their position in the original video
- **Trimmed Clip Discovery**: Find shorter, highlight versions of longer gameplay

## 🏗️ Architecture

### Tech Stack
- **Backend**: Axum Rust web framework with SQLx connection pooling
- **Frontend**: Next.js 14+ with TypeScript
- **Authentication**: NextAuth.js with Google OAuth
- **Storage**: MinIO object storage with presigned URLs
- **Database**: PostgreSQL with pgvector for embeddings + SQLx pooling
- **Queue System**: Redis for background job processing
- **Video Processing**: FFmpeg for transcoding and thumbnail generation
- **AI/ML**: OpenAI embeddings for content similarity and search
- **Algorithms**: Dynamic Time Warping (DTW) for trimmed clip detection

### Production Architecture
- **Stateless API Design**: Axum backend with integrated SQLx pools scales horizontally
- **Direct Database Pooling**: Built-in SQLx connection pooling for optimal performance
- **Direct Storage Access**: Frontend uploads directly to MinIO via presigned URLs
- **Background Processing**: Worker pool processes videos from MinIO storage
- **Linear Scaling**: Add Axum instances behind load balancer for unlimited scale
- **Multi-Frontend Support**: Separated backend enables mobile/desktop clients

### System Requirements
- **Minimum Hardware**: GTX 1060, Intel CPU (older generation compatible)
- **Storage**: MinIO cluster or single instance with sufficient space
- **Network**: Self-hosted deployment with Docker Compose
- **Infrastructure**: PostgreSQL with connection pooling, Redis, MinIO for production scaling
- **Database Connections**: Managed database recommended for 20-30 concurrent connections per Axum instance

## 🚀 Getting Started

### Prerequisites
- **Docker & Docker Compose** for container orchestration
- **Rust toolchain** for backend development
- **Node.js 18+** for frontend development
- **MinIO** for object storage (included in Docker Compose)
- **PostgreSQL** with pgvector extension
- **Redis** for job queuing

### Quick Start with Docker Compose

1. Clone the repository:
```bash
git clone https://github.com/MassimoNodin/clipstream.git
cd clipstream
```

2. Start the complete stack:
```bash
docker-compose up -d
```

This will start:
- MinIO object storage (localhost:9000)
- PostgreSQL database
- Redis job queue  
- Axum API backend
- Next.js frontend (localhost:3000)

### Development Setup

#### Backend (Axum)
```bash
cd api
cargo run
```

#### Frontend (Next.js)
```bash
cd web/clipstream
npm install
npm run dev
```

#### MinIO Setup
Access MinIO console at http://localhost:9001 and create buckets:
- `raw-uploads` - Original video files
- `processed-videos` - Transcoded HLS segments
- `thumbnails` - Video thumbnails
- `transcripts` - Speech-to-text results
- `embeddings` - AI analysis data

### Environment Variables

#### Backend (.env)
```env
DATABASE_URL=postgresql://user:pass@localhost:5432/clipstream
REDIS_URL=redis://localhost:6379
MINIO_ENDPOINT=http://localhost:9000
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=minioadmin
OPENAI_API_KEY=your-openai-key
JWT_SECRET=your-jwt-secret
```

#### Frontend (.env.local)
```env
NEXTAUTH_SECRET=your-secret-key
NEXTAUTH_URL=http://localhost:3000
NEXT_PUBLIC_API_BASE_URL=http://localhost:8080
GOOGLE_CLIENT_ID=your-google-client-id
GOOGLE_CLIENT_SECRET=your-google-client-secret
```

## ⚡ Horizontal Scaling Architecture

### Built-in SQLx Connection Pooling
Each Axum instance includes integrated connection pooling for optimal database performance:

```rust
// Each Axum instance maintains its own connection pool
let pool = PgPoolOptions::new()
    .max_connections(20)              // Per-instance connection limit
    .min_connections(5)               // Always-ready connections
    .acquire_timeout(Duration::from_secs(8))
    .idle_timeout(Duration::from_secs(600))
    .max_lifetime(Duration::from_secs(3600))
    .connect(&database_url)
    .await?;
```

### Linear Scaling Strategy
- **Development**: Single Axum instance with 10-connection pool
- **Production**: Multiple Axum instances behind load balancer
- **High Scale**: 10+ instances, each with 20-30 connections
- **Database**: Managed PostgreSQL handling 200-400 total connections

### Scaling Benefits
- **No Central Bottleneck**: Each instance manages its own database connections
- **Fault Tolerance**: Instance failure doesn't affect connection pooling
- **Performance**: Direct database access without additional network hops
- **Simplicity**: Single service deployment with built-in optimization

### Load Balancer Configuration
```yaml
# Example with multiple Axum instances
services:
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
    depends_on:
      - api-1
      - api-2
      - api-3

  api-1:
    build: ./api
    environment:
      - DATABASE_URL=postgresql://user:pass@managed-db:5432/clipstream
  
  api-2:
    build: ./api
    environment:
      - DATABASE_URL=postgresql://user:pass@managed-db:5432/clipstream
      
  api-3:
    build: ./api
    environment:
      - DATABASE_URL=postgresql://user:pass@managed-db:5432/clipstream
```

## 🔧 Development
```

## 📱 Usage

### Stream Management
- **Create Streams**: Set up dedicated spaces for different games or groups
- **Invite Users**: Send invites with specific role permissions
- **Role Assignment**: Manage who can view, upload, or administrate

### Uploading Clips
- **Supported Formats**: MP4, AVI, MOV, and other common video formats
- **Automatic Processing Pipeline**: Every uploaded video goes through a 5-stage processing workflow
- **Duplicate Detection**: Automatic checking for exact duplicates before processing
- **Multi-bitrate Transcoding**: Videos are transcoded into multiple bitrates for optimal streaming
- **Metadata Extraction**: Speech-to-text and thumbnail generation
- **AI Analysis**: Embedding generation for similarity, POV, and trimmed clip detection

### Discovery Features
- **Search**: Find clips by spoken content or metadata
- **Similar Clips**: Browse AI-suggested related content
- **Timeline View**: Interactive timeline showing all trimmed clips positioned relative to the original video
- **POV Detection**: Discover the same moment from different perspectives

## 🔄 Video Processing Pipeline with MinIO

Every uploaded video goes through a comprehensive 5-stage processing workflow integrated with MinIO object storage:

### MinIO Storage Integration
- **Direct Uploads**: Frontend uploads directly to MinIO via presigned URLs
- **Webhook Triggers**: MinIO notifies API when uploads complete
- **Worker Processing**: Background workers download from MinIO, process, and upload results
- **Direct Streaming**: Users stream processed videos directly from MinIO buckets

### Processing Stages with Storage
- **Index 1 - Duplicate Detection**: 
  - Downloads metadata from `raw-uploads/{video_id}.mp4`
  - Compares file hashes without full download
  - If duplicate found: Processing index set to -1 (flagged as duplicate)
  - If unique: Proceed to next stage

- **Index 2 - Video Transcoding**: 
  - Downloads raw video from MinIO `raw-uploads/` bucket
  - Transcodes to multiple bitrates for adaptive streaming
  - Uploads HLS segments to `processed-videos/{video_id}/` 
  - Generates thumbnails to `thumbnails/{video_id}.jpg`

- **Index 3 - Speech-to-Text**: 
  - Extracts audio from MinIO-stored video
  - Transcribes spoken content for search functionality
  - Saves transcripts to `transcripts/{video_id}.json`

- **Index 4 - AI Analysis**: 
  - Downloads processed video for embedding generation
  - Detects similar clips, POVs, and trimmed versions using embeddings
  - Stores analysis results in `embeddings/{video_id}.json`

- **Index 0 - Complete**: Video fully processed with all assets in MinIO

### Processing Status with Storage Paths
- `1`: Duplicate detection (accessing `raw-uploads/`)
- `2`: Video transcoding (creating `processed-videos/`, `thumbnails/`)
- `3`: Speech-to-text processing (creating `transcripts/`)
- `4`: AI analysis (creating `embeddings/`)
- `0`: Processing complete (all assets available in MinIO)
- `-1`: Flagged as duplicate

## 🔧 Development

### Project Structure
```
clipstream/
├── web/clipstream/          # Next.js application
│   ├── src/
│   │   ├── app/             # App router pages
│   │   ├── components/      # Shared components
│   │   └── lib/             # Utilities
│   └── public/              # Static assets
└── README.md                # This file
```

### Key Features in Development
- [ ] Video upload and processing pipeline with 5-stage workflow
- [ ] Duplicate detection system
- [ ] Multi-bitrate transcoding pipeline
- [ ] Speech-to-text processing
- [ ] Embedding generation for similarity detection
- [ ] DTW algorithm for trimmed clip detection
- [ ] Timeline view with aligned clip visualization
- [ ] POV detection algorithm
- [ ] Stream invitation system
- [ ] Role-based access control
- [ ] Search functionality with speech-to-text
- [ ] Social features (likes, shares)

## 🎯 Roadmap

### Phase 1 - Core Platform
- [x] Next.js setup with authentication
- [ ] Stream creation and management
- [ ] Video upload and storage
- [ ] Basic video playback

### Phase 2 - Intelligence Features
- [ ] Video transcoding pipeline
- [ ] Speech-to-text processing
- [ ] Embedding generation
- [ ] DTW algorithm implementation
- [ ] Similar clip detection
- [ ] Timeline view interface

### Phase 3 - Advanced Features
- [ ] POV detection
- [ ] Advanced search
- [ ] Social features
- [ ] Performance optimizations

## 🤝 Contributing

This is currently a personal project designed for scalability. Contributions, issues, and feature requests are welcome!

1. Fork the project
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## ⚡ Performance Notes

Optimized for modest hardware:
- Efficient video processing on GTX 1060
- Smart caching for transcoded videos
- Optimized embedding calculations
- Progressive loading for large clip libraries

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Built with Next.js and the React ecosystem
- Video processing powered by FFmpeg
- UI components from shadcn/ui
- Authentication via NextAuth.js

---

**Note**: Clipstream is designed as a self-hosted solution for gaming communities who want full control over their content and data.