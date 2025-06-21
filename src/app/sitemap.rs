use crate::shared::*;
use chrono::{DateTime, Utc};
use momenta::{nodes::Element, prelude::*};
use ngyn::macros::handler;
use ngyn::shared::server::NgynResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct SitemapProps {
    pub posts: Vec<Post>,
    pub static_pages: Vec<StaticPage>,
    pub base_url: String,
}

#[component]
pub fn SitemapPage(props: &SitemapProps) -> Node {
    let now = Utc::now().format("%Y-%m-%dT%H:%M:%S+00:00").to_string();

    // Static pages from the application
    let app_pages = vec![
        ("/", "1.0", "daily"),
        ("/essays", "0.9", "daily"),
        ("/projects", "0.8", "weekly"),
        ("/resume", "0.7", "monthly"),
        ("/adventures", "0.7", "weekly"),
        ("/newsletter", "0.6", "monthly"),
    ];

    let sitemap_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}
{}
{}
</urlset>"#,
        // Application pages
        app_pages
            .iter()
            .map(|(path, priority, changefreq)| {
                format!(
                    r#"  <url>
    <loc>{}{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>{}</changefreq>
    <priority>{}</priority>
  </url>"#,
                    props.base_url.trim_end_matches('/'),
                    path,
                    now,
                    changefreq,
                    priority
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        // Blog posts
        props
            .posts
            .iter()
            .map(|post| {
                let lastmod = post
                    .updated_at
                    .as_ref()
                    .or(post.published_at.as_ref())
                    .and_then(|date_str| DateTime::parse_from_rfc3339(date_str).ok())
                    .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S+00:00").to_string())
                    .unwrap_or_else(|| now.clone());

                format!(
                    r#"  <url>
    <loc>{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>"#,
                    post.url, lastmod
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        // Static pages from CMS
        props
            .static_pages
            .iter()
            .map(|page| {
                format!(
                    r#"  <url>
    <loc>{}/{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.6</priority>
  </url>"#,
                    props.base_url.trim_end_matches('/'),
                    page.slug,
                    now
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    );

    Element::parse_tag_with_attributes(
        "",
        "",
        Default::default(),
        Default::default(),
        &sitemap_content,
        Default::default(),
    )
}

#[handler]
pub async fn sitemap_handler(res: &mut NgynResponse) -> String {
    res.headers_mut().insert(
        "Content-Type",
        "application/xml; charset=utf-8".parse().unwrap(),
    );
    res.headers_mut()
        .insert("Cache-Control", "public, max-age=3600".parse().unwrap());

    if let Ok(SitemapQuery { publication }) = HASHNODE_CLIENT
        .execute_query(
            SITEMAP_QUERY.to_owned(),
            Some(json!({
                "host": "elcharitas.wtf/blog",
                "staticPagesCount": 20,
                "postsCount": 100,
            })),
        )
        .await
    {
        let posts = publication
            .posts
            .edges
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|edge| edge.node.clone())
            .collect();

        let static_pages = publication
            .static_pages
            .edges
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|edge| edge.node.clone())
            .collect();

        let props = SitemapProps {
            posts,
            static_pages,
            base_url: publication.url,
        };

        return SitemapPage::render(&props).to_string();
    }

    // Fallback sitemap content
    let fallback_props = SitemapProps {
        posts: Vec::new(),
        static_pages: Vec::new(),
        base_url: "https://elcharitas.wtf".to_string(),
    };

    SitemapPage::render(&fallback_props).to_string()
}
