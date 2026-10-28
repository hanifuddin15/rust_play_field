use axum::{
    http::{header, HeaderValue, Method},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod state;

use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::new();

    // Configure CORS for local frontend development (e.g. running dx serve on 8080 or Vite)
    let cors = CorsLayer::new()
        .allow_origin([
            "http://127.0.0.1:8080".parse::<HeaderValue>()?,
            "http://localhost:8080".parse::<HeaderValue>()?,
            "http://127.0.0.1:3000".parse::<HeaderValue>()?,
            "http://localhost:3000".parse::<HeaderValue>()?,
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::ACCEPT]);

    // REST API routes under /api
    let api_routes = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/todos", get(handlers::list_todos).post(handlers::create_todo))
        .route(
            "/todos/:id",
            get(handlers::get_todo)
                .put(handlers::update_todo)
                .delete(handlers::delete_todo),
        );

    // Serve static files from Dioxus dist bundle when deployed or tested in production mode
    let static_service =
        ServeDir::new("crates/frontend/dist").append_index_html_on_directories(true);

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(static_service)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("🦀 Axum backend running on http://{}", addr);
    tracing::info!("👉 Health check: http://{}/api/health", addr);
    tracing::info!("👉 Todos API:   http://{}/api/todos", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

