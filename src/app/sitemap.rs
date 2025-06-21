use crate::shared::*;
use chrono::Utc;
use momenta::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct SitemapProps {
    pub posts: Vec<Post>,
    pub static_pages: Vec<StaticPage>,
    pub base_url: String,
}

impl Default for SitemapProps {
    fn default() -> Self {
        Self {
            posts: Vec::new(),
            static_pages: Vec::new(),
            base_url: String::from("https://elcharitas.wtf"),
        }
    }
}

impl PageLoader for SitemapProps {
    async fn load(ctx: &mut ngyn::shared::server::NgynContext<'_>) -> Self {
        ctx.response_mut().headers_mut().insert(
            "Content-Type",
            "application/xml; charset=utf-8".parse().unwrap(),
        );
        ctx.response_mut()
            .headers_mut()
            .insert("Cache-Control", "public, max-age=3600".parse().unwrap());

        HASHNODE_CLIENT
            .execute_query::<SitemapQuery>(
                SITEMAP_QUERY.to_owned(),
                Some(json!({
                    "host": "elcharitas.wtf/blog",
                    "staticPagesCount": 20,
                    "postsCount": 100,
                })),
            )
            .await
            .unwrap();

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
            let SitemapPublication {
                posts,
                static_pages,
                url,
                ..
            } = publication;
            let posts = posts
                .edges
                .unwrap_or_default()
                .into_iter()
                .map(|edge| edge.node)
                .collect();

            let static_pages = static_pages
                .edges
                .unwrap_or_default()
                .into_iter()
                .map(|edge| edge.node)
                .collect();

            return SitemapProps {
                posts,
                static_pages,
                base_url: url,
            };
        }
        SitemapProps::default()
    }
}

#[component]
pub fn SitemapPage(props: &SitemapProps) -> Node {
    use xml_elements as momenta;
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

    rsx! {
        <sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
            <sitemap>
                <loc>{&props.base_url}/sitemap.xml</loc>
                <lastmod>{&now}</lastmod>
            </sitemap>
            {app_pages.iter().map(|(loc, priority, changefreq)| {
                 rsx! {
                    <sitemap>
                        <loc>{&props.base_url}{loc}</loc>
                        <lastmod>{&now}</lastmod>
                        <changefreq>{changefreq}</changefreq>
                        <priority>{priority}</priority>
                    </sitemap>
                 }
             })}
            {props.posts.iter().map(|post| {
                rsx! {
                    <sitemap>
                        <loc>{&props.base_url}/essays/{&post.slug}</loc>
                        <lastmod>{post.updated_at.as_ref().map_or("", |v| v)}</lastmod>
                        <changefreq>"daily"</changefreq>
                        <priority>"0.5"</priority>
                    </sitemap>
                }
            })}
            {props.static_pages.iter().map(|page| {
                rsx! {
                    <sitemap>
                        <loc>{&props.base_url}{&page.slug}</loc>
                        <lastmod>{&now}</lastmod>
                        <changefreq>"monthly"</changefreq>
                        <priority>"0.5"</priority>
                    </sitemap>
                }
            })}
        </sitemapindex>
    }
}
