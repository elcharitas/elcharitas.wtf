mod adventures {
    mod adventure;
    mod page;
}
mod blog {
    mod page;
    mod slug;
}
mod home;
mod newsletter;
mod projects {
    mod page;
    mod slug;
}

use home::HomePage;
use ngyn::prelude::*;

use crate::shared::route_handler;

pub fn register_routes(app: &mut HyperApplication) {
    // Pages
    app.get("/", route_handler(HomePage));

    // Redirects
    app.get("/blog", redirect_permanent("/writings")); // Old blog URL
    app.get("/mods/resume", redirect_permanent("/resume")); // Old resume URL
    app.get("/mods/connect", redirect_permanent("/connect"));
    app.get("/connect", redirect_to("https://cal.com/elcharitas"));
    app.get(
        "/resume",
        redirect_to(
            "https://docs.google.com/document/d/1DwEEKQcO5RLs0bA55XTM-hOj4jYWQTTJlvZ7RD9tD-w/edit",
        ),
    );
}
