use crate::shared::*;
use chrono::{DateTime, Utc};
use momenta::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Default, Serialize, Deserialize)]
pub struct RSSProps {
    pub posts: Vec<Post>,
    pub publication: Option<Publication>,
}

impl PageLoader for RSSProps {
    async fn load(ctx: &mut ngyn::shared::server::NgynContext<'_>) -> Self {
        ctx.response_mut().headers_mut().insert(
            "Content-Type",
            "application/xml; charset=utf-8".parse().unwrap(),
        );
        ctx.response_mut()
            .headers_mut()
            .insert("Cache-Control", "public, max-age=3600".parse().unwrap());

        if let Ok(RSSFeedQuery { publication }) = HASHNODE_CLIENT
            .execute_query(
                RSS_QUERY.to_owned(),
                Some(json!({
                    "host": "elcharitas.wtf/blog",
                    "first": 50,
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
                .collect::<Vec<_>>();

            return RSSProps {
                posts,
                publication: Some(publication),
            };
        }
        RSSProps::default()
    }
}

#[component]
pub fn RSSPage(props: &RSSProps) -> Node {
    use xml_elements as momenta;
    let now = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    rsx! {
        <rss>
            <channel>
                <title>{&props.publication.as_ref().map(|p| p.title.to_string()).unwrap_or_default()}</title>
                <link>{&props.publication.as_ref().map(|p| p.url.to_string()).unwrap_or_default()}</link>
                <description>
                    {&props
                        .publication.as_ref().and_then(|p| p.description_seo.clone()).unwrap_or_default()}
                </description>
                <lastBuildDate>{&now}</lastBuildDate>
                {&props
                    .posts
                    .iter()
                    .map(|post| {
                        let pub_date = post
                            .published_at
                            .as_ref()
                            .and_then(|date_str| DateTime::parse_from_rfc3339(date_str).ok())
                            .map(|dt| dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string())
                            .unwrap_or_else(|| now.clone());

                        rsx! {
                            <item>
                                <title>{&post.title}</title>
                                <link>{&post.url}</link>
                                <guid>{&post.url}</guid>
                                <description>{&post.brief}</description>
                                <pubDate>{&pub_date}</pubDate>
                            </item>
                        }
                    })}
            </channel>
        </rss>
    }
}
