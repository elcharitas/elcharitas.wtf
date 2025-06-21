use crate::shared::*;
use chrono::{DateTime, Utc};
use momenta::{nodes::Element, prelude::*};
use ngyn::macros::handler;
use ngyn::shared::server::NgynResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct RSSProps {
    pub posts: Vec<Post>,
    pub publication: Publication,
}

#[component]
pub fn RSSPage(props: &RSSProps) -> Node {
    let now = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
    let base_url = "https://elcharitas.wtf";

    let rss_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>{}</title>
    <link>{}</link>
    <description>{}</description>
    <language>en-us</language>
    <lastBuildDate>{}</lastBuildDate>
    <atom:link href="{}/rss.xml" rel="self" type="application/rss+xml" />
{}
  </channel>
</rss>"#,
        props.publication.title,
        props.publication.url,
        props
            .publication
            .description_seo
            .as_deref()
            .unwrap_or("Personal blog and portfolio of Jonathan Irhodia (elcharitas)"),
        now,
        base_url,
        props
            .posts
            .iter()
            .map(|post| {
                let pub_date = post
                    .published_at
                    .as_ref()
                    .and_then(|date_str| DateTime::parse_from_rfc3339(date_str).ok())
                    .map(|dt| dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string())
                    .unwrap_or_else(|| now.clone());

                format!(
                    r#"    <item>
      <title><![CDATA[{}]]></title>
      <link>{}</link>
      <guid>{}</guid>
      <description><![CDATA[{}]]></description>
      <pubDate>{}</pubDate>
    </item>"#,
                    post.title, post.url, post.url, post.brief, pub_date
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
        &rss_content,
        Default::default(),
    )
}

#[handler]
pub async fn rss_handler(res: &mut NgynResponse) -> String {
    res.headers_mut().insert(
        "Content-Type",
        "application/rss+xml; charset=utf-8".parse().unwrap(),
    );
    res.headers_mut()
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

        let props = RSSProps { posts, publication };

        return RSSPage::render(&props).to_string();
    }

    // Fallback RSS content
    let fallback_props = RSSProps {
        posts: Vec::new(),
        publication: Publication {
            id: String::new(),
            title: "elcharitas.wtf".to_string(),
            display_title: Some("elcharitas.wtf".to_string()),
            url: "https://elcharitas.wtf".to_string(),
            meta_tags: None,
            favicon: None,
            is_team: false,
            followers_count: None,
            description_seo: Some(
                "Personal blog and portfolio of Jonathan Irhodia (elcharitas)".to_string(),
            ),
            posts: PublicationPostConnection {
                total_documents: 0,
                edges: None,
                page_info: None,
            },
            author: User::default(),
            og_meta_data: OpenGraphMetaData { image: None },
            preferences: Preferences {
                logo: None,
                dark_mode: None,
                navbar_items: Vec::new(),
            },
            links: None,
            integrations: None,
        },
    };

    RSSPage::render(&fallback_props).to_string()
}
