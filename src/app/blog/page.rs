use crate::components::{article::Article, PageLayout};
use crate::shared::*;
use momenta::prelude::*;
use ngyn::shared::server::Transformer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BlogProps {
    pub posts: Vec<Post<u64>>,
    pub cursor: Option<String>,
}

impl<'a> Transformer<'a> for BlogProps {
    fn transform(_: &'a mut ngyn::shared::server::NgynContext) -> Self {
        BlogProps {
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
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-12">
                    {Vec::<Post<u64>>::new().iter().map(|post| {
                        rsx! {<Article post={post.clone()} show_read_more />}
                    })}
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    {posts.iter().map(|post| {
                        rsx! {<Article post={post.clone()} show_read_more />}
                    })}
                </div>
            </div>
        </PageLayout>
    }
}
