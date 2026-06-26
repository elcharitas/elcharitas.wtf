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
    let mut seen_cats = std::collections::HashSet::new();
    let categories: Vec<String> = posts
        .iter()
        .filter_map(|p| p.tags.first().map(|t| t.name.clone()))
        .filter(|c| seen_cats.insert(c.clone()))
        .collect();

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

                <div class="space-y-4">
                    <div class="relative">
                        <i class="fas fa-search absolute left-4 top-1/2 -translate-y-1/2 text-zinc-500 text-sm pointer-events-none"></i>
                        <input
                            id="search-input"
                            type="text"
                            placeholder="Search essays..."
                            class="w-full bg-zinc-900/60 border border-zinc-800 rounded-xl pl-10 pr-4 py-3 text-sm text-zinc-200 placeholder-zinc-600 focus:outline-none focus:border-zinc-700 transition-colors"
                        />
                    </div>
                    <div class="flex flex-wrap gap-2">
                        <button data_tag_filter="" class="text-xs px-3 py-1.5 rounded-full border border-zinc-700 text-zinc-400 hover:border-zinc-500 cursor-pointer transition-colors">"All"</button>
                        {categories.iter().map(|cat| {
                            rsx! {
                                <button data_tag_filter={cat.as_str()} class="text-xs px-3 py-1.5 rounded-full border border-zinc-700 text-zinc-500 hover:border-zinc-500 cursor-pointer transition-colors capitalize">{cat.replace('-', " ")}</button>
                            }
                        })}
                    </div>
                </div>

                <div id="click_to_load_rows" class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
                    {posts.into_iter().map(|post| {
                        let tag_name = post.tags.first().map_or_else(String::new, |t| t.name.clone());
                        let search_text = format!("{} {}", post.title, post.brief);
                        rsx! {
                            <div data_searchtext={search_text.as_str()} data_tags={tag_name.as_str()}>
                                <Article post={post.clone()} show_read_more />
                            </div>
                        }
                    })}
                </div>

                <script>{r#"
                (function(){
                  var q='',activeTag=null;
                  function filter(){
                    document.querySelectorAll('[data-searchtext]').forEach(function(el){
                      var text=(el.getAttribute('data-searchtext')||'').toLowerCase();
                      var tags=(el.getAttribute('data-tags')||'').toLowerCase();
                      var ms=!q||text.includes(q);
                      var mt=!activeTag||tags.split(',').some(function(t){return t.trim()===activeTag;});
                      el.style.display=ms&&mt?'':'none';
                    });
                  }
                  var s=document.getElementById('search-input');
                  if(s)s.addEventListener('input',function(){q=s.value.toLowerCase().trim();filter();});
                  function setPill(el){
                    document.querySelectorAll('[data-tag-filter]').forEach(function(p){p.removeAttribute('style');});
                    if(el)el.style.cssText='background:var(--accent-dim);border-color:var(--accent-border);color:var(--accent);';
                  }
                  document.querySelectorAll('[data-tag-filter]').forEach(function(pill){
                    pill.addEventListener('click',function(){
                      activeTag=pill.getAttribute('data-tag-filter')||null;
                      setPill(pill);
                      filter();
                    });
                  });
                  setPill(document.querySelector('[data-tag-filter=""]'));
                })();
                "#}</script>
            </div>
        </PageLayout>
    }
}

pub async fn infinite_scroll(Query(_query): Query<serde_json::Value>) -> impl IntoResponse {
    ([("Content-Type", "text/event-stream")], String::new())
}
