mod adventures {
    pub mod page;
}
mod blog {
    pub mod page;
    pub mod slug;
}
mod error;
mod home;
mod newsletter;
mod projects;
mod resume {
    pub mod page;
}
mod rss;
mod sitemap;

use home::HomePage;
use ngyn::prelude::*;

use crate::shared::route_handler;

pub fn register_routes(app: &mut HyperApplication) {
    // Pages
    app.get("/", route_handler(HomePage));
    app.get("/resume", route_handler(resume::page::ResumePage));
    app.get("/essays", route_handler(blog::page::BlogPage));
    app.get(
        "/essays/infinite_scroll",
        async_wrap(blog::page::infinite_scroll),
    );
    app.get("/essays/{slug}", route_handler(blog::slug::BlogDetailPage));
    app.get("/projects", route_handler(projects::ProjectsPage));
    app.get(
        "/projects/infinite_scroll",
        async_wrap(projects::infinite_scroll),
    );
    app.get(
        "/adventures",
        route_handler(adventures::page::AdventuresPage),
    );
    app.any("/newsletter", route_handler(newsletter::NewsletterPage));

    // RSS and Sitemap
    app.get("/rss.xml", async_wrap(rss::rss_handler));
    app.get("/sitemap.xml", async_wrap(sitemap::sitemap_handler));

    // Redirects
    app.get("/blog", redirect_permanent("/essays")); // Old blog URL
    app.get("/mods/resume", redirect_permanent("/resume")); // Old resume URL
    app.get("/mods/connect", redirect_permanent("/connect"));
    app.get("/connect", redirect_to("https://cal.com/elcharitas"));
    app.get(
        "/resume.docx",
        redirect_to(
            "https://docs.google.com/document/d/e/2PACX-1vScEJP53Da_tdWTzCNBvfMDMCjy43udlv5YDhM81YowUOFxwDaEPyjuAXTZppupRoS0a9KVWUbaiy2p/pub?embedded=true",
        ),
    );

    // 404
    app.any("/{*path}", route_handler(error::ErrorPage));
}
