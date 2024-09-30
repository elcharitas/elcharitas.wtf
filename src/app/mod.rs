mod adventures;
mod blog;
mod home;
mod newsletter;
mod projects;

use home::HomePage;
use ngyn::prelude::*;

pub fn register_routes(app: &mut HyperApplication) {
    // Pages
    app.get("/", handler(HomePage::route_handler));

    // Redirects
    app.get("/blog", redirect_permanent("/writings")); // Old blog URL
    app.get("/mods/resume", redirect_permanent("/resume")); // Old resume URL
    app.get(
        "/resume",
        redirect_to(
            "https://docs.google.com/document/d/1DwEEKQcO5RLs0bA55XTM-hOj4jYWQTTJlvZ7RD9tD-w/edit",
        ),
    );
}
