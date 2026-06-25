use crate::requests::fetch_all_posts;
use crate::shared::*;
use axum::response::IntoResponse;
use chrono::Utc;
use momenta::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct RSSProps {
    pub posts: Vec<Post>,
}

impl RSSProps {
    async fn load() -> Self {
        Self {
            posts: fetch_all_posts().await,
        }
    }
}

pub async fn rss_handler() -> impl IntoResponse {
    let props = RSSProps::load().await;
    (
        [
            ("Content-Type", "application/rss+xml; charset=utf-8"),
            ("Cache-Control", "public, max-age=3600"),
        ],
        RSSPage::render(&props).to_string(),
    )
}

#[component]
pub fn RSSPage(props: &RSSProps) -> Node {
    use xml_elements as momenta;
    let now = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    rsx! {
        <rss>
            <channel>
                <title>"elcharitas.wtf — Essays"</title>
                <link>"https://elcharitas.wtf/essays"</link>
                <description>"Software development, product decisions, and the realities of shipping."</description>
                <lastBuildDate>{&now}</lastBuildDate>
                {&props.posts.iter().map(|post| {
                    rsx! {
                        <item>
                            <title>{&post.title}</title>
                            <link>{&post.url}</link>
                            <guid>{&post.url}</guid>
                            <description>{&post.brief}</description>
                            <pubDate>{&now}</pubDate>
                        </item>
                    }
                })}
            </channel>
        </rss>
    }
}
