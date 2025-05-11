mod app;
mod components;
mod shared;

use app::register_routes;
use ngyn::prelude::*;

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();

    register_routes(&mut app);

    let _ = app.listen("0.0.0.0:3000").await;
}
