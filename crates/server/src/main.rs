use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use janus_llm::MockLlmClient;
use janus_server::{create_router, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,janus_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting JanusRP Server...");

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    // Optional DB connection for Slice 1.1 / development
    let db_url = std::env::var("DATABASE_URL").ok();
    let db_pool = if let Some(url) = db_url {
        match janus_db::create_pool(&url).await {
            Ok(pool) => {
                info!("Database connected");
                if let Err(e) = janus_db::run_migrations(&pool).await {
                    tracing::error!("Failed to run database migrations: {}", e);
                }
                Some(pool)
            }
            Err(e) => {
                tracing::warn!("Could not connect to PostgreSQL (will run without DB): {}", e);
                None
            }
        }
    } else {
        None
    };

    let llm = Arc::new(MockLlmClient::new());
    let state = AppState::new(db_pool, llm);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = create_router(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
