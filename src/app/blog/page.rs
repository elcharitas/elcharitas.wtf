use crate::components::card::ScrollCard;
use crate::components::{PageLayout, article::Article};
use crate::shared::*;
use axum::{
    extract::Query,
    response::{Html, IntoResponse},
};
use momenta::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct BlogProps {
    pub posts: Vec<PostEdge>,
    pub cursor: Option<String>,
    pub has_next_page: Option<bool>,
}

impl BlogProps {
    async fn load() -> Self {
        if let Ok(PostsByPublicationQuery { publication }) = HASHNODE_CLIENT
            .execute_query(
                POSTS_QUERY.to_owned(),
                Some(json!({
                    "host": "elcharitas.wtf/blog",
                    "first": 15,
                })),
            )
            .await
        {
            let PublicationPostConnection {
                edges, page_info, ..
            } = publication.posts;

            let PageInfo {
                end_cursor,
                has_next_page,
            } = page_info.unwrap_or_default();

            return Self {
                posts: edges.unwrap_or_default(),
                cursor: end_cursor,
                has_next_page,
            };
        }

        Self {
            posts: Vec::new(),
            cursor: None,
            has_next_page: Some(false),
        }
    }
}

pub async fn blog_handler() -> impl IntoResponse {
    let props = BlogProps::load().await;
    Html(BlogPage::render(&props).to_string())
}

#[component]
pub fn BlogPage(
    BlogProps {
        posts,
        cursor,
        has_next_page,
    }: &BlogProps,
) -> Node {
    let total_posts = posts.len() as u32;
    rsx! {
        <PageLayout title="Essays">
            <div class="py-4 md:py-8 space-y-8" data_signals={format!("{{'cursor': '{}', 'has_next_page': {}}}", cursor.as_ref().map_or("", |v| v), has_next_page.unwrap_or(false))}>
                <section class="space-y-4">
                    <h1 class="text-4xl md:text-5xl font-semibold text-white">"Essays"</h1>
                    <div class="section-rule"></div>
                    <p class="text-base text-zinc-300 max-w-3xl">
                        "I write about software development, product decisions, productivity, and the realities of shipping. "
                        <span class="text-sm" style="color: var(--accent); opacity: 0.7;">"("{total_posts}" loaded)"</span>
                    </p>
                </section>

                <div id="click_to_load_rows" class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4" data_fragment_merge_target="$has_next_page">
                    {posts.into_iter().map(|post| {
                        rsx! {<Article post={post.node.clone()} show_read_more />}
                    })}
                </div>
                <ScrollCard intersect="@get('/essays/infinite_scroll')" />
            </div>
        </PageLayout>
    }
}

pub async fn infinite_scroll(Query(query): Query<serde_json::Value>) -> impl IntoResponse {
    let page_query = PageQuery::from_query(Query(query));
    let PageQuery {
        cursor,
        has_next_page,
        ..
    } = page_query;

    if !has_next_page {
        return ([("Content-Type", "text/event-stream")], String::new());
    }

    if let Ok(PostsByPublicationQuery { publication }) = HASHNODE_CLIENT
        .execute_query(
            POSTS_QUERY.to_owned(),
            Some(json!({
                "host": "elcharitas.wtf/blog",
                "first": 15,
                "after": cursor
            })),
        )
        .await
    {
        let posts = publication.posts;
        let PageInfo {
            end_cursor,
            has_next_page,
        } = posts.page_info.unwrap_or_default();

        let posts: Vec<Post> = posts
            .edges
            .unwrap_or_default()
            .iter()
            .map(|edge| edge.node.clone())
            .collect();
        let fragment = rsx! {
            <>
                {posts.iter().map(|post| {
                    rsx! {
                        <>
                            "event: datastar-merge-fragments\n"
                            "data: selector #click_to_load_rows\n"
                            "data: mergeMode append\n"
                            "data: fragments " <Article post={post.clone()} show_read_more />
                            "\n\n"
                        </>
                    }
                })}
                "event: datastar-merge-signals\n"
                "data: onlyIfMissing false\n"
                {format!("data: signals {{'cursor': '{}', 'has_next_page': {}}}\n\n", end_cursor.as_ref().map_or("", |v| v), has_next_page.unwrap_or(false))}
            </>
        };

        return (
            [("Content-Type", "text/event-stream")],
            fragment.to_string(),
        );
    }
    ([("Content-Type", "text/event-stream")], String::new())
}
