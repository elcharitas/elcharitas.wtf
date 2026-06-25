use crate::requests::fetch_all_posts;
use crate::shared::*;
use axum::response::IntoResponse;
use chrono::Utc;
use momenta::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SitemapProps {
    pub posts: Vec<Post>,
    pub base_url: String,
}

impl Default for SitemapProps {
    fn default() -> Self {
        Self {
            posts: Vec::new(),
            base_url: String::from("https://elcharitas.wtf"),
        }
    }
}

impl SitemapProps {
    async fn load() -> Self {
        Self {
            posts: fetch_all_posts().await,
            base_url: "https://elcharitas.wtf".to_string(),
        }
    }
}

pub async fn sitemap_handler() -> impl IntoResponse {
    let props = SitemapProps::load().await;
    (
        [
            ("Content-Type", "application/xml; charset=utf-8"),
            ("Cache-Control", "public, max-age=3600"),
        ],
        SitemapPage::render(&props).to_string(),
    )
}

#[component]
pub fn SitemapPage(props: &SitemapProps) -> Node {
    use xml_elements as momenta;
    let now = Utc::now().format("%Y-%m-%dT%H:%M:%S+00:00").to_string();

    let app_pages = vec![
        ("/", "1.0", "daily"),
        ("/essays", "0.9", "daily"),
        ("/projects", "0.8", "weekly"),
        ("/publications", "0.8", "monthly"),
        ("/resume", "0.7", "monthly"),
        ("/adventures", "0.7", "weekly"),
        ("/newsletter", "0.6", "monthly"),
    ];

    rsx! {
        <sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
            {app_pages.iter().map(|(path, priority, changefreq)| {
                rsx! {
                    <sitemap>
                        <loc>{format!("{}{}", &props.base_url, path)}</loc>
                        <lastmod>{&now}</lastmod>
                        <changefreq>{changefreq}</changefreq>
                        <priority>{priority}</priority>
                    </sitemap>
                }
            })}
            {props.posts.iter().map(|post| {
                rsx! {
                    <sitemap>
                        <loc>{format!("{}/essays/{}", &props.base_url, &post.slug)}</loc>
                        <lastmod>{&now}</lastmod>
                        <changefreq>"monthly"</changefreq>
                        <priority>"0.7"</priority>
                    </sitemap>
                }
            })}
        </sitemapindex>
    }
}
