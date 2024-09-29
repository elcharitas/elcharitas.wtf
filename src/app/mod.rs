mod adventures;
mod home;
mod newsleter;
mod projects;

use home::Home;
use ngyn::prelude::*;

pub fn register_routes(app: &mut HyperApplication) {
    app.get("/", handler(Home::route_handler));
    app.get("/blog", redirect_permanent("/writings"));
    app.get(
        "/resume",
        redirect_to(
            "https://docs.google.com/document/d/1DwEEKQcO5RLs0bA55XTM-hOj4jYWQTTJlvZ7RD9tD-w/edit",
        ),
    );
}
