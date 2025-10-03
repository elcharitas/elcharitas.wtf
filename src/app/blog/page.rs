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
    rsx! {
        <PageLayout title="Essays">
            <div class="container mx-auto px-4 py-12" data_signals={format!("{{'cursor': '{}', 'has_next_page': {}}}", cursor.as_ref().map_or("", |v| v), has_next_page.unwrap_or(false))}>
                <div class="text-center mb-12">
                    <h1 class="text-3xl md:text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-yellow-400 to-orange-400 mb-4">
                        "Essays"
                    </h1>
                    <p class="text-base text-zinc-300 max-w-2xl mx-auto">
                        "I write about my experiences and thoughts on how to do software development, productivity, and life."
                    </p>
                </div>
                <div id="click_to_load_rows" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8" data_fragment_merge_target="$has_next_page">
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
