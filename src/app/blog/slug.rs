use crate::components::PageLayout;
use crate::requests::fetch_post_by_slug_from_github;
use crate::shared::*;
use axum::{
    extract::Path,
    http::HeaderMap,
    response::{Html, IntoResponse},
};
use comrak::{Options, markdown_to_html, ExtensionOptions, RenderOptions};
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
                Some(post) => {
                    let category = post.tags.first().map(|t| t.name.as_str()).unwrap_or("general");
                    let md_opts = Options {
                        render: RenderOptions { unsafe_: true, ..Default::default() },
                        extension: ExtensionOptions { table: true, strikethrough: true, autolink: true, tasklist: true, ..Default::default() },
                        ..Default::default()
                    };
                    let html_content = post.content.as_ref().map(|c| markdown_to_html(&c.markdown, &md_opts)).unwrap_or_default();
                    rsx! {
                        <article class="max-w-3xl mx-auto py-6 md:py-10">
                            <header class="mb-10 space-y-4">
                                <a href="/essays" class="inline-flex items-center gap-2 text-sm text-zinc-500 hover:text-white transition-colors mb-2">
                                    <i class="fas fa-arrow-left text-xs"></i>
                                    <span>"All Essays"</span>
                                </a>
                                <div class="flex items-center gap-3">
                                    <span class="inline-flex items-center px-2.5 py-1 text-[11px] uppercase tracking-[0.08em] font-semibold rounded-md" style="background: var(--accent-dim); border: 1px solid var(--accent-border); color: var(--accent);">
                                        {category}
                                    </span>
                                    <span class="text-xs text-zinc-500">{format!("{} min read", post.read_time_in_minutes)}</span>
                                </div>
                                <h1 class="text-3xl md:text-5xl font-bold text-white leading-tight">
                                    {&post.title}
                                </h1>
                                <p class="text-lg text-zinc-400 leading-relaxed">{&post.brief}</p>
                                <div class="section-rule"></div>
                            </header>

                            <div class="essay-body">
                                <div _dangerously_set_inner_html={html_content} />
                            </div>

                            <footer class="mt-16 pt-8 border-t border-zinc-800 space-y-12">
                                <div id="comments-section">
                                    <h2 class="text-xl font-semibold text-white mb-6">"Comments"</h2>
                                    <div id="HCB_comment_box"></div>
                                    <link rel="stylesheet" type="text/css" href="https://www.htmlcommentbox.com/static/skins/bootstrap/twitter-bootstrap.css?v=0" />
                                    <script type="text/javascript" id="hcb" _dangerously_set_inner_html={r#"if(!window.hcb_user){hcb_user={};} (function(){var s=document.createElement("script"), l=hcb_user.PAGE || (""+window.location).replace(/'/g,"%27"), h="https://www.htmlcommentbox.com";s.setAttribute("type","text/javascript");s.setAttribute("src", h+"/jread?page="+encodeURIComponent(l).replace("+","%2B")+"&mod=%241%24wq1rdBcg%24Dxb22lGTzRNjgQtkjW3c4%2F"+"&opts=16798&num=10&ts=1782379931181");if (typeof s!="undefined") document.getElementsByTagName("head")[0].appendChild(s);})();"#} />
                                    <script _dangerously_set_inner_html={r#"
(function() {
    var HIDE = [
        '.home-desc', '#hcb-footer', '.hcb-footer', '.hcb-branding',
        '[class*="powered"]', '[class*="htmlcommentbox"]'
    ];
    function purge(root) {
        HIDE.forEach(function(sel) {
            root.querySelectorAll(sel).forEach(function(el) {
                el.style.setProperty('display', 'none', 'important');
                el.style.setProperty('height', '0', 'important');
                el.style.setProperty('overflow', 'hidden', 'important');
            });
        });
        // also hide first h3 which is typically "HTML Comment Box"
        var h3s = root.querySelectorAll('#HCB_comment_box h3');
        if (h3s.length) h3s[0].style.setProperty('display', 'none', 'important');
    }
    var box = document.getElementById('HCB_comment_box');
    if (!box) return;
    purge(box);
    var obs = new MutationObserver(function() { purge(box); });
    obs.observe(box, { childList: true, subtree: true });
})();
                                    "#} />
                                </div>
                            </footer>
                        </article>
                    }
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
