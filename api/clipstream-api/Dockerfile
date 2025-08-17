# Use the official Rust image as base
FROM rust:1.89 as builder

# Create app directory
WORKDIR /usr/src/app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

# Copy the binary from builder stage
COPY --from=builder /usr/src/app/target/release/clipstream-api /usr/local/bin/clipstream-api

# Change ownership to app user
RUN chown appuser:appuser /usr/local/bin/clipstream-api

# Switch to app user
USER appuser

# Expose port
EXPOSE 8000

# Run the binary
CMD ["clipstream-api"]
