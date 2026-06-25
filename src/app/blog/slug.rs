use crate::components::PageLayout;
use crate::requests::fetch_post_by_slug_from_github;
use crate::shared::*;
use axum::{
    extract::Path,
    http::HeaderMap,
    response::{Html, IntoResponse},
};
use comrak::{Options, markdown_to_html};
use momenta::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct BlogDetailProps {
    pub post: Option<Post>,
    pub slug: String,
}

impl BlogDetailProps {
    async fn load(slug: String) -> Self {
        let post = fetch_post_by_slug_from_github(&slug).await;
        Self { post, slug }
    }
}

pub async fn blog_detail_handler(
    Path(params): Path<PageParams>,
    _headers: HeaderMap,
) -> impl IntoResponse {
    let props = BlogDetailProps::load(params.slug).await;
    Html(BlogDetailPage::render(&props).to_string())
}

#[component]
pub fn BlogDetailPage(props: &BlogDetailProps) -> Node {
    let title = props
        .post
        .as_ref()
        .map_or("Blog Post", |p| &p.title)
        .to_string();
    rsx! {
        <PageLayout title={title}>
            {match &props.post {
                Some(post) => rsx! {
                    <article class="max-w-4xl mx-auto">
                        <header class="mb-8">
                            <h1 class="text-3xl md:text-5xl font-bold text-white mb-4 leading-tight">
                                {&post.title}
                            </h1>
                            <div class="flex items-center gap-4 text-sm text-zinc-400 mb-6">
                                <span>{format!("{} min read", post.read_time_in_minutes)}</span>
                            </div>
                        </header>

                        <div class="prose prose-invert prose-lg max-w-none mb-12">
                            <div class="text-zinc-200 leading-relaxed">
                               {when!(let Some(content) = &post.content => <p _dangerously_set_inner_html={markdown_to_html(&content.markdown, &Options::default())} />)}
                            </div>
                        </div>

                        <footer class="border-t border-zinc-700 pt-8 mb-12">
                            <a
                                href="/essays"
                                class="inline-flex items-center space-x-2 px-6 py-3 bg-zinc-800 text-white rounded-lg hover:bg-zinc-700 transition-colors font-medium border border-zinc-700"
                            >
                                <i class="fas fa-arrow-left"></i>
                                <span>"Back to Essays"</span>
                            </a>
                        </footer>
                    </article>
                },
                None => rsx! {
                    <div class="max-w-2xl mx-auto text-center py-16">
                        <i class="fas fa-search text-6xl text-zinc-600 mb-4"></i>
                        <h1 class="text-3xl font-bold text-white mb-4">"Article Not Found"</h1>
                        <p class="text-zinc-400 mb-8">
                            "The article you're looking for doesn't exist or has been moved."
                        </p>
                        <a
                            href="/essays"
                            class="inline-flex items-center space-x-2 px-6 py-3 bg-zinc-800 text-white rounded-lg hover:bg-zinc-700 transition-colors font-medium border border-zinc-700"
                        >
                            <i class="fas fa-arrow-left"></i>
                            <span>"Go back"</span>
                        </a>
                    </div>
                }
            }}
        </PageLayout>
    }
}
