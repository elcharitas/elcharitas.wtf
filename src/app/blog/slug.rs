use crate::{
    analytics::{
        get_or_create_device_id, send_views_to_hashnode_analytics_dashboard,
        send_views_to_hashnode_internal_analytics,
    },
    components::PageLayout,
    shared::*,
};
use comrak::{markdown_to_html, Options};
use momenta::prelude::*;
use ngyn::{http::HeaderMap, prelude::*, shared::server::Transformer};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Default, Serialize, Deserialize)]
pub struct BlogDetailProps {
    pub post: Option<Post>,
    pub related_posts: Vec<Post>,
    pub slug: String,
}

impl PageLoader for BlogDetailProps {
    async fn load(ctx: &mut NgynContext<'_>) -> Self {
        let headers = ctx.request().headers().clone();
        let PageParams { slug } = PageParams::transform(ctx);

        let post = fetch_post_by_slug(&slug, headers).await;
        let related_posts = if post.is_some() {
            fetch_related_posts(&slug, 3).await
        } else {
            Vec::new()
        };

        Self {
            post,
            related_posts,
            slug,
        }
    }
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
                        // Hero Section with Cover Image
                        {when!(let Some(cover_url) = &post.cover_image =>
                            <div class="relative w-full h-64 md:h-96 mb-8 rounded-xl overflow-hidden">
                                <img
                                    src={&cover_url.url}
                                    alt={format!("Cover image for {}", post.title)}
                                    class="w-full h-full object-cover"
                                />
                                <div class="absolute inset-0 bg-gradient-to-t from-zinc-900/60 to-transparent"></div>
                            </div>
                        )}

                        // Article Header
                        <header class="mb-8">
                            <div class="flex flex-wrap items-center gap-2 mb-4">
                                {post.tags.iter().map(|tag| {
                                    rsx! {
                                        <span class="inline-flex items-center px-3 py-1 text-xs font-medium bg-zinc-800/60 text-zinc-300 rounded-full border border-zinc-700/50">
                                            {&tag.name}
                                        </span>
                                    }
                                })}
                            </div>

                            <h1 class="text-3xl md:text-5xl font-bold text-white mb-4 leading-tight">
                                {&post.title}
                            </h1>

                            <div class="flex items-center justify-between text-sm text-zinc-400 mb-6">
                                <div class="flex items-center space-x-6">
                                    {when!(let Some(published_at) = &post.published_at =>
                                        <time
                                            datetime={&*published_at}
                                            class="flex items-center space-x-2"
                                        >
                                            <i class="far fa-calendar-alt"></i>
                                            <span>
                                                {chrono::DateTime::parse_from_rfc3339(&published_at).unwrap_or_default().format("%B %d, %Y").to_string()}
                                            </span>
                                        </time>
                                    )}

                                    <div class="flex items-center space-x-2">
                                        <i class="far fa-eye text-orange-400/60"></i>
                                        <span>{post.views} views</span>
                                    </div>

                                    <div class="flex items-center space-x-2">
                                        <i class="far fa-comment text-yellow-400/60"></i>
                                        <span>{post.comments.total_documents} comments</span>
                                    </div>
                                </div>

                                <span class="text-zinc-500">5 min read</span>
                            </div>
                        </header>

                        // Article Content
                        <div class="prose prose-invert prose-lg max-w-none mb-12">
                            <div class="text-zinc-200 leading-relaxed">
                               {when!(let Some(content) = &post.content => <p _dangerously_set_inner_html={markdown_to_html(&content.markdown, &Options::default())} />)}
                            </div>
                        </div>

                        // Article Footer
                        <footer class="border-t border-zinc-700 pt-8 mb-12">
                            <div class="flex items-center justify-between">
                                <div class="flex items-center space-x-4">
                                    <span class="text-sm text-zinc-400">Share this article:</span>
                                    <div class="flex space-x-3">
                                        <button class="p-2 rounded-lg bg-zinc-800 text-zinc-300 hover:bg-zinc-700 hover:text-white transition-colors">
                                            <i class="fab fa-twitter text-sm"></i>
                                        </button>
                                        <button class="p-2 rounded-lg bg-zinc-800 text-zinc-300 hover:bg-zinc-700 hover:text-white transition-colors">
                                            <i class="fab fa-linkedin text-sm"></i>
                                        </button>
                                        <button class="p-2 rounded-lg bg-zinc-800 text-zinc-300 hover:bg-zinc-700 hover:text-white transition-colors">
                                            <i class="fab fa-facebook text-sm"></i>
                                        </button>
                                    </div>
                                </div>

                                <button class="flex items-center space-x-2 px-4 py-2 bg-zinc-800 text-zinc-300 rounded-lg hover:bg-zinc-700 hover:text-white transition-colors">
                                    <i class="far fa-bookmark text-sm"></i>
                                    <span class="text-sm">Save</span>
                                </button>
                            </div>
                        </footer>

                        // Related Posts Section
                        {when!(!props.related_posts.is_empty() =>
                            <section class="border-t border-zinc-700 pt-12">
                                <h2 class="text-2xl font-bold text-white mb-6">Related Articles</h2>
                                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
                                    {props.related_posts.iter().map(|related_post| {
                                        rsx! {
                                            <a
                                                href={format!("/writings/{}", related_post.slug)}
                                                class="group block bg-zinc-900 rounded-lg overflow-hidden border border-zinc-700 hover:border-zinc-600 transition-colors"
                                            >
                                                {when!(let Some(cover_url) = &related_post.cover_image =>
                                                    <div class="aspect-video overflow-hidden">
                                                        <img
                                                            src={&cover_url.url}
                                                            alt={format!("Cover image for {}", related_post.title)}
                                                            class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
                                                        />
                                                    </div>
                                                )}
                                                <div class="p-4">
                                                    <h3 class="font-semibold text-white group-hover:text-yellow-400 transition-colors mb-2 line-clamp-2">
                                                        {&related_post.title}
                                                    </h3>
                                                    <p class="text-sm text-zinc-400 line-clamp-2">
                                                        {&related_post.brief}
                                                    </p>
                                                </div>
                                            </a>
                                        }
                                    })}
                                </div>
                            </section>
                        )}

                        // Comments Section
                        <section class="border-t border-zinc-700 pt-12">
                            <h2 class="text-2xl font-bold text-white mb-6">
                               "Comments: "{post.comments.total_documents}
                            </h2>

                            // Comment Form
                            <div class="bg-zinc-900/50 rounded-lg p-6 mb-8">
                                <form class="space-y-4">
                                    <div>
                                        <label for="comment" class="block text-sm font-medium text-zinc-300 mb-2">
                                            Leave a comment
                                        </label>
                                        <textarea
                                            id="comment"
                                            name="comment"
                                            rows={4}
                                            class="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 rounded-lg text-white placeholder-zinc-400 focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:border-transparent"
                                            placeholder="Share your thoughts..."
                                        ></textarea>
                                    </div>
                                    <div class="flex items-center justify-between">
                                        <p class="text-xs text-zinc-500">
                                            Comments are moderated and will be published after review.
                                        </p>
                                        <button
                                            type="submit"
                                            class="px-6 py-2 bg-yellow-600 text-white rounded-lg hover:bg-yellow-700 transition-colors font-medium"
                                        >
                                            Post Comment
                                        </button>
                                    </div>
                                </form>
                            </div>

                            // TODO: Render existing comments here
                            <div class="space-y-6">
                                <p class="text-zinc-400 text-center py-8">
                                    Be the first to comment on this article!
                                </p>
                            </div>
                        </section>
                    </article>
                },
                None => rsx! {
                    <div class="max-w-2xl mx-auto text-center py-16">
                        <div class="mb-8">
                            <i class="fas fa-search text-6xl text-zinc-600 mb-4"></i>
                            <h1 class="text-3xl font-bold text-white mb-4">Article Not Found</h1>
                            <p class="text-zinc-400 mb-8">
                                "The article you're looking for doesn't exist or has been moved."
                            </p>
                            <a
                                href="/writings"
                                class="inline-flex items-center space-x-2 px-6 py-3 bg-yellow-600 text-white rounded-lg hover:bg-yellow-700 transition-colors font-medium"
                            >
                                <i class="fas fa-arrow-left"></i>
                                <span>Go back</span>
                            </a>
                        </div>
                    </div>
                }
            }}
        </PageLayout>
    }
}

async fn fetch_post_by_slug(slug: &str, headers: HeaderMap) -> Option<Post> {
    if let Ok(SinglePostByPublicationQuery { publication }) = HASHNODE_CLIENT
        .execute_query::<SinglePostByPublicationQuery>(
            SINGLE_POST_QUERY.to_owned(),
            Some(json!({
                "slug": slug,
                "host": "elcharitas.wtf/blog",
            })),
        )
        .await
    {
        let (device_id, _) = get_or_create_device_id("");

        tokio::spawn(send_views_to_hashnode_internal_analytics(
            publication.clone(),
            headers.clone(),
            device_id,
        ));

        tokio::spawn(send_views_to_hashnode_analytics_dashboard(
            publication.clone(),
            headers,
        ));
        return publication.and_then(|publ| publ.post);
    }
    None
}

async fn fetch_related_posts(_current_slug: &str, _limit: usize) -> Vec<Post> {
    Vec::new()
}
