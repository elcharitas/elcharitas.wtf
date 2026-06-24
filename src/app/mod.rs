#[cfg(not(target_arch = "wasm32"))]
mod adventures {
    pub mod page;
}
#[cfg(not(target_arch = "wasm32"))]
mod blog {
    pub mod page;
    pub mod slug;
}
mod error;
mod home;
#[cfg(not(target_arch = "wasm32"))]
mod newsletter;
#[cfg(not(target_arch = "wasm32"))]
mod projects;
mod resume {
    pub mod page;
}
#[cfg(not(target_arch = "wasm32"))]
mod rss;
#[cfg(not(target_arch = "wasm32"))]
mod sitemap;

use axum::{
    Router,
    routing::get,
};
#[cfg(not(target_arch = "wasm32"))]
use axum::routing::post;

pub fn create_router() -> Router {
    let router = Router::new()
        .route("/", get(home::home_handler))
        .route("/resume", get(resume::page::resume_handler));

    #[cfg(not(target_arch = "wasm32"))]
    let router = router
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
        .route("/rss.xml", get(rss::rss_handler))
        .route("/sitemap.xml", get(sitemap::sitemap_handler))
        .route(
            "/blog",
            get(|| async { axum::response::Redirect::permanent("/essays") }),
        );

    #[cfg(target_arch = "wasm32")]
    let router = router.route(
        "/blog",
        get(|| async { axum::response::Redirect::permanent("/") }),
    );

    router
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
                    "https://docs.google.com/document/d/e/2PACX-1vRxPZLfm3NkCml3MhdrnEIBjTBzhRvDznOyDYF19SGUlMaijecJ8oAtcFI-dnp49vC3Ndt2NJvlssNA/pub?embedded=true",
                )
            }),
        )
        .fallback(error::error_handler)
}
