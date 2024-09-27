mod adventures;
mod home;
mod newsleter;
mod projects;

use home::Home;
use ngyn::prelude::*;

pub fn register_routes(app: &mut HyperApplication) {
    app.get("/", handler(Home::route_handler));
}
