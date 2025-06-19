mod analytics;
mod app;
mod components;
mod requests;
mod shared;

use app::register_routes;
use ngyn::{prelude::*, shared::core::NgynPlatform};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let _ = dotenv::dotenv();

    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter("info,elcharitas=debug")
        .init();

    tracing::info!("Starting elcharitas.wtf server");

    let mut app = HyperApplication::default();

    register_routes(&mut app);

    app.use_static(std::path::Path::new("public").to_path_buf())?;
    tracing::info!("Static files directory configured");

    app.on_error(|err| {
        tracing::error!("Server error: {:?}", err);
    });

    tracing::info!("Server listening on http://0.0.0.0:3000");
    app.listen("0.0.0.0:3000").await;
    Ok(())
}
