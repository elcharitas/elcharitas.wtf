mod app;
mod components;
mod public;
mod shared;

use app::register_routes;
use ngyn::prelude::*;
use ngyn_shuttle::{ShuttleApplication, ShuttleNgyn};
use public::static_files;

#[shuttle_runtime::main]
async fn main() -> ShuttleNgyn {
    let mut app = ShuttleApplication::default();

    register_routes(&mut app);

    // static files
    app.get("/*", static_files);

    Ok(app.into())
}
