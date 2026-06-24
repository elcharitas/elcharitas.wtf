mod assets;
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
    routing::get,
};
#[cfg(not(target_arch = "wasm32"))]
use axum::routing::post;
#[cfg(target_arch = "wasm32")]
use axum::{
    extract::{Path, Query},
    http::{Method, Request},
    response::{IntoResponse, Response},
};

pub fn create_router() -> Router {
    let router = Router::new()
        .route("/", get(home::home_handler))
        .route("/resume", get(resume::page::resume_handler))
        .route("/styles.css", get(assets::styles_handler))
        .route("/CalSans-SemiBold.woff2", get(assets::calsans_font_handler))
        .route("/favicon.png", get(assets::favicon_handler))
        .route("/og.png", get(assets::og_image_handler));

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

#[cfg(target_arch = "wasm32")]
pub async fn wasm_dynamic_response<B>(req: &Request<B>) -> Option<Response> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let headers = req.headers().clone();

    let mut query_map = serde_json::Map::new();
    if let Some(query) = req.uri().query() {
        for (k, v) in url::form_urlencoded::parse(query.as_bytes()) {
            query_map.insert(k.to_string(), serde_json::Value::String(v.to_string()));
        }
    }
    let query_value = serde_json::Value::Object(query_map);

    if method == Method::GET && path == "/essays" {
        return Some(blog::page::blog_handler().await.into_response());
    }
    if method == Method::GET && path == "/essays/infinite_scroll" {
        return Some(
            blog::page::infinite_scroll(Query(query_value))
                .await
                .into_response(),
        );
    }
    if method == Method::GET && path.starts_with("/essays/") {
        let slug = path.trim_start_matches("/essays/").to_string();
        return Some(
            blog::slug::blog_detail_handler(Path(crate::shared::PageParams { slug }), headers)
                .await
                .into_response(),
        );
    }

    if method == Method::GET && path == "/projects" {
        return Some(projects::projects_handler().await.into_response());
    }
    if method == Method::GET && path == "/projects/infinite_scroll" {
        return Some(
            projects::infinite_scroll(Query(query_value))
                .await
                .into_response(),
        );
    }

    if method == Method::GET && path == "/adventures" {
        return Some(adventures::page::adventures_handler().await.into_response());
    }
    if method == Method::GET && path == "/newsletter" {
        return Some(newsletter::newsletter_get_handler().await.into_response());
    }
    if method == Method::GET && path == "/rss.xml" {
        return Some(rss::rss_handler().await.into_response());
    }
    if method == Method::GET && path == "/sitemap.xml" {
        return Some(sitemap::sitemap_handler().await.into_response());
    }

    None
}
