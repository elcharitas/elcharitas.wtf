use crate::components::card::ScrollCard;
use crate::components::{PageLayout, article::ProjectArticle};
use crate::requests::get_all_projects;
use crate::shared::{PageQuery, Project};
use axum::{
    extract::Query,
    response::{Html, IntoResponse},
};
use momenta::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

thread_local! {
    static PROJECTS: RefCell<Vec<Project>> = RefCell::new(Vec::new());
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

    let page: usize = cursor.parse().unwrap_or(1);
    let projects = get_all_projects(page as u32).await.unwrap_or_default();
    let has_next_page = !projects.is_empty();

    let fragment = rsx! {
        <>
            {projects.into_iter().map(|project| {
                let search_text = format!("{} {}", project.name, project.description);
                let tags_str = project.tags.join(",");
                rsx! {
                    <>
                        "event: datastar-merge-fragments\n"
                        "data: selector #click_to_load_rows\n"
                        "data: mergeMode append\n"
                        "data: fragments "
                        <div data_searchtext={search_text.as_str()} data_tags={tags_str.as_str()}>
                            <ProjectArticle {..project} />
                        </div>
                        "\n\n"
                    </>
                }
            })}
            "event: datastar-merge-signals\n"
            "data: onlyIfMissing false\n"
            {format!("data: signals {{'cursor': '{}', 'has_next_page': {}}}\n\n", page + 1, has_next_page)}
        </>
    };

    (
        [("Content-Type", "text/event-stream")],
        fragment.to_string(),
    )
}

#[derive(Serialize, Deserialize)]
pub struct ProjectsProps {
    pub projects: Vec<Project>,
    pub cursor: Option<String>,
    pub has_next_page: Option<bool>,
}

impl ProjectsProps {
    async fn load() -> Self {
        let mut cached_projects = PROJECTS.with_borrow(|projects| projects.clone());

        if cached_projects.is_empty() {
            cached_projects = get_all_projects(1).await.unwrap_or_default();
            PROJECTS.with_borrow_mut(|stored| {
                *stored = cached_projects.clone();
            });
        }

        Self {
            projects: cached_projects,
            cursor: Some("2".to_string()),
            has_next_page: Some(true),
        }
    }
}

pub async fn projects_handler() -> impl IntoResponse {
    let props = ProjectsProps::load().await;
    Html(ProjectsPage::render(&props).to_string())
}

#[component]
pub fn ProjectsPage(
    ProjectsProps {
        projects,
        cursor,
        has_next_page,
    }: &ProjectsProps,
) -> Node {
    let mut seen_tags = std::collections::HashSet::new();
    let all_tags: Vec<String> = projects
        .iter()
        .flat_map(|p| p.tags.iter().cloned())
        .filter(|t| !t.is_empty() && seen_tags.insert(t.clone()))
        .take(20)
        .collect();

    rsx! {
        <PageLayout title="Projects">
            <div class="py-4 md:py-8 space-y-8" data_signals={format!("{{'cursor': '{}', 'has_next_page': {}}}", cursor.as_ref().map_or("1", |c| c), has_next_page.unwrap_or(false))}>
                <section class="space-y-4">
                    <h1 class="text-4xl md:text-5xl font-semibold text-white">"Projects"</h1>
                    <div class="section-rule"></div>
                    <p class="text-base text-zinc-300 max-w-3xl">
                        "Open-source contributions and personal experiments across backend systems, tools, and product prototypes. "
                    </p>
                </section>

                <div class="space-y-4">
                    <div class="relative">
                        <i class="fas fa-search absolute left-4 top-1/2 -translate-y-1/2 text-zinc-500 text-sm pointer-events-none"></i>
                        <input
                            id="search-input"
                            type="text"
                            placeholder="Search projects..."
                            class="w-full bg-zinc-900/60 border border-zinc-800 rounded-xl pl-10 pr-4 py-3 text-sm text-zinc-200 placeholder-zinc-600 focus:outline-none focus:border-zinc-700 transition-colors"
                        />
                    </div>
                    {if !all_tags.is_empty() {
                        rsx! {
                            <div class="flex flex-wrap gap-2">
                                <button data_tag_filter="" class="text-xs px-3 py-1.5 rounded-full border border-zinc-700 text-zinc-400 hover:border-zinc-500 cursor-pointer transition-colors">"All"</button>
                                {all_tags.iter().map(|tag| {
                                    rsx! {
                                        <button data_tag_filter={tag.as_str()} class="text-xs px-3 py-1.5 rounded-full border border-zinc-700 text-zinc-500 hover:border-zinc-500 cursor-pointer transition-colors">{tag.replace('-', " ")}</button>
                                    }
                                })}
                            </div>
                        }
                    } else {
                        rsx! { <></> }
                    }}
                </div>

                <div id="click_to_load_rows" class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4" data_fragment_merge_target="$has_next_page">
                    {projects.into_iter().map(|project| {
                        let search_text = format!("{} {}", project.name, project.description);
                        let tags_str = project.tags.join(",");
                        rsx! {
                            <div data_searchtext={search_text.as_str()} data_tags={tags_str.as_str()}>
                                <ProjectArticle {..project.clone()} />
                            </div>
                        }
                    })}
                </div>
                <ScrollCard intersect="@get('/projects/infinite_scroll')" />

                <script>{r#"
                (function(){
                  var q='',activeTag=null;
                  function filter(){
                    var hasFilter=q||activeTag;
                    var sc=document.querySelector('[data-intersect]');
                    if(sc)sc.style.display=hasFilter?'none':'';
                    document.querySelectorAll('[data-searchtext]').forEach(function(el){
                      var text=(el.getAttribute('data-searchtext')||'').toLowerCase();
                      var tags=(el.getAttribute('data-tags')||'').toLowerCase();
                      var ms=!q||text.includes(q);
                      var mt=!activeTag||tags.split(',').some(function(t){return t.trim()===activeTag;});
                      el.style.display=ms?(mt?'':'none'):'none';
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
