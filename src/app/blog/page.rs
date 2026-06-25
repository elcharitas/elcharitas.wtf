use crate::components::{PageLayout, article::Article};
use crate::requests::fetch_all_posts;
use crate::shared::*;
use axum::{
    extract::Query,
    response::{Html, IntoResponse},
};
use momenta::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BlogProps {
    pub posts: Vec<Post>,
}

impl BlogProps {
    async fn load() -> Self {
        Self {
            posts: fetch_all_posts().await,
        }
    }
}

pub async fn blog_handler() -> impl IntoResponse {
    let props = BlogProps::load().await;
    Html(BlogPage::render(&props).to_string())
}

#[component]
pub fn BlogPage(BlogProps { posts }: &BlogProps) -> Node {
    let total_posts = posts.len() as u32;
    rsx! {
        <PageLayout title="Essays">
            <div class="py-4 md:py-8 space-y-8">
                <section class="space-y-4">
                    <h1 class="text-4xl md:text-5xl font-semibold text-white">"Essays"</h1>
                    <div class="section-rule"></div>
                    <p class="text-base text-zinc-300 max-w-3xl">
                        "I write about software development, product decisions, productivity, and the realities of shipping. "
                    </p>
                </section>

                <div id="click_to_load_rows" class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
                    {posts.into_iter().map(|post| {
                        rsx! {<Article post={post.clone()} show_read_more />}
                    })}
                </div>
            </div>
        </PageLayout>
    }
}

pub async fn infinite_scroll(Query(_query): Query<serde_json::Value>) -> impl IntoResponse {
    ([("Content-Type", "text/event-stream")], String::new())
}
