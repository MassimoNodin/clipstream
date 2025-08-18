use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

mod auth;
mod streams;
mod invites;
mod videos;
mod search;
mod processing;
mod files;
mod admin;

#[derive(Debug, Serialize)]
struct HealthStatus {
    status: String,
    database: String,
    pool_size: u32,
    pool_idle: usize,
    pool_connections: usize,
}

#[derive(Debug, Serialize)]
struct AppError {
    error: String,
}

// Health check handler that uses the database pool
async fn health_check(State(pool): State<PgPool>) -> Result<Json<HealthStatus>, StatusCode> {
    // Test database connection
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => Ok(Json(HealthStatus {
            status: "healthy".to_string(),
            database: "connected".to_string(),
            pool_size: pool.size(),
            pool_idle: pool.num_idle(),
            pool_connections: pool.size() as usize - pool.num_idle(),
        })),
        Err(_) => Ok(Json(HealthStatus {
            status: "unhealthy".to_string(),
            database: "disconnected".to_string(),
            pool_size: pool.size(),
            pool_idle: pool.num_idle(),
            pool_connections: pool.size() as usize - pool.num_idle(),
        })),
    }
}

// Simple hello world handler
async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get database URL from environment variable or use default
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://clipstream:password@localhost:5432/clipstream".to_string());

    // Create connection pool with optimal settings for production
    let pool = PgPoolOptions::new()
        .max_connections(20)              // Maximum connections in pool
        .min_connections(5)               // Always-ready connections
        .acquire_timeout(Duration::from_secs(8))  // Timeout for getting connection
        .idle_timeout(Duration::from_secs(600))   // Close idle connections after 10 minutes
        .max_lifetime(Duration::from_secs(3600))  // Recreate connections every hour
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    println!("Database connection pool established");
    println!("Pool size: {}", pool.size());

    // Build application with routes and shared state
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check))
        .merge(auth::routes())
        .merge(streams::routes())
        .merge(invites::routes())
        .merge(videos::routes())
        .merge(search::routes())
        .merge(processing::routes())
        .merge(files::routes())
        .merge(admin::routes())
        .with_state(pool); // Share the pool across all routes

    // Create server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://0.0.0.0:8000");
    println!("Health check available at http://0.0.0.0:8000/health");

    // Run the server
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");

    Ok(())
}