mod adventures;
mod home;
mod newsleter;
mod projects;

use home::Home;
use ngyn::{prelude::*, shared::core::redirect_to};

pub fn register_routes(app: &mut HyperApplication) {
    app.get("/", handler(Home::route_handler));
    app.get("/blog", redirect_to("/writings"))
}
