-- Drop existing tables if they exist (for development purposes)
DROP TABLE IF EXISTS stream_members CASCADE;
DROP TABLE IF EXISTS videos CASCADE;
DROP TABLE IF EXISTS streams CASCADE;
DROP TABLE IF EXISTS users CASCADE;

-- Extension for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Streams (communities/groups)
CREATE TABLE streams (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_by UUID NOT NULL REFERENCES users(id),
    is_private BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Stream membership and permissions
CREATE TABLE stream_members (
    stream_id UUID NOT NULL REFERENCES streams(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL, -- 'owner', 'admin', 'creator', 'viewer'
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (stream_id, user_id)
);

-- Videos table (core functionality)
CREATE TABLE videos (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    stream_id UUID NOT NULL REFERENCES streams(id) ON DELETE CASCADE,
    uploader_id UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    duration_seconds INTEGER,
    storage_path VARCHAR(255),
    thumbnail_path VARCHAR(255),
    processing_status VARCHAR(50) NOT NULL DEFAULT 'pending', -- 'pending', 'processing', 'completed', 'failed', 'duplicate'
    processing_progress FLOAT DEFAULT 0,
    views_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Basic indexes
CREATE INDEX idx_videos_stream_id ON videos(stream_id);
CREATE INDEX idx_videos_uploader_id ON videos(uploader_id);
CREATE INDEX idx_videos_processing_status ON videos(processing_status);