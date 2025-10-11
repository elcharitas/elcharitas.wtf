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

use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router() -> Router {
    Router::new()
        // Pages
        .route("/", get(home::home_handler))
        .route("/resume", get(resume::page::resume_handler))
        .route("/essays", get(blog::page::blog_handler))
        .route("/essays/infinite_scroll", get(blog::page::infinite_scroll))
        .route("/essays/{slug}", get(blog::slug::blog_detail_handler))
        .route("/projects", get(projects::projects_handler))
        .route(
            "/projects/infinite_scroll",
            get(projects::infinite_scroll),
        )
        .route("/adventures", get(adventures::page::adventures_handler))
        .route("/newsletter", get(newsletter::newsletter_get_handler))
        .route("/newsletter", post(newsletter::newsletter_post_handler))
        // RSS and Sitemap
        .route("/rss.xml", get(rss::rss_handler))
        .route("/sitemap.xml", get(sitemap::sitemap_handler))
        // Redirects
        .route(
            "/blog",
            get(|| async { axum::response::Redirect::permanent("/essays") }),
        )
        .route(
            "/mods/resume",
            get(|| async { axum::response::Redirect::permanent("/resume") }),
        )
        .route(
            "/mods/connect",
            get(|| async { axum::response::Redirect::permanent("/connect") }),
        )
        .route(
            "/connect",
            get(|| async { axum::response::Redirect::temporary("https://cal.com/elcharitas") }),
        )
        .route(
            "/resume.docx",
            get(|| async {
                axum::response::Redirect::temporary(
                    "https://docs.google.com/document/d/14QvZPgD1Ahouu84Fc2HwPlXVP8rUGcy0lM0LuK5uc8s/edit",
                )
            }),
        )
        // 404
        .fallback(error::error_handler)
}
