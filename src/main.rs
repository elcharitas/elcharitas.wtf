mod analytics;
mod app;
mod components;
mod requests;
mod shared;

use app::create_router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter("info,elcharitas=debug")
        .init();

    tracing::info!("Starting elcharitas.wtf server");

    let app = create_router()
        .fallback_service(ServeDir::new("public"))
        .layer(TraceLayer::new_for_http());

    tracing::info!("Static files directory configured");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server listening on http://0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
