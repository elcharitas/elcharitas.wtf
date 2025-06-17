use crate::components::{article::Article, PageLayout};
use crate::shared::*;
use momenta::prelude::*;
use ngyn::shared::server::NgynContext;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct BlogProps {
    pub posts: Vec<Post>,
    pub cursor: Option<String>,
}

impl PageLoader for BlogProps {
    async fn load(_ctx: &mut NgynContext<'_>) -> Self {
        if let Ok(PostsByPublicationQuery { publication }) = HASHNODE_CLIENT
            .execute_query(
                POSTS_QUERY.to_owned(),
                Some(json!({
                    "host": "elcharitas.wtf/blog",
                    "first": 12
                })),
            )
            .await
        {
            return Self {
                posts: publication
                    .posts
                    .edges
                    .unwrap_or_default()
                    .iter()
                    .map(|edge| edge.node.clone())
                    .collect(),
                cursor: None,
            };
        }

        Self {
            posts: Vec::new(),
            cursor: None,
        }
    }
}

#[component]
pub fn BlogPage(BlogProps { posts, .. }: &BlogProps) -> Node {
    rsx! {
        <PageLayout title="Writings">
            <div class="container mx-auto px-4 py-12">
                <h1 class="text-4xl font-bold text-zinc-50 mb-8">"Blogs ✍🏼"</h1>
                <p class="text-lg text-zinc-200 mb-12">
                    "I write about my experiences and thoughts on how to do software development, productivity, and life."
                </p>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {posts.iter().map(|post| {
                        rsx! {<Article post={post.clone()} show_read_more />}
                    })}
                </div>
            </div>
        </PageLayout>
    }
}
