mod app;
mod components;
mod public;
mod shared;

use app::register_routes;
use ngyn::prelude::*;
use public::static_files;

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();

    register_routes(&mut app);

    // static files
    app.get("/*", static_files);

    let _ = app.listen("0.0.0.0:3000").await;
}
