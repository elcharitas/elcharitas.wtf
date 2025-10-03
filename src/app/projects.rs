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
                rsx! {
                    <>
                        "event: datastar-merge-fragments\n"
                        "data: selector #click_to_load_rows\n"
                        "data: mergeMode append\n"
                        "data: fragments " <ProjectArticle {..project} />
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
    rsx! {
        <PageLayout title="Projects">
            <div class="container mx-auto px-4 py-12" data_signals={format!("{{'cursor': '{}', 'has_next_page': {}}}", cursor.as_ref().map_or("1", |c| c), has_next_page.unwrap_or(false))}>
                <div class="text-center mb-12">
                    <h1 class="text-3xl md:text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-yellow-400 to-orange-400 mb-4">
                        "Projects"
                    </h1>
                    <p class="text-base text-zinc-300 max-w-2xl mx-auto">
                        "Here are some of the projects I've worked on, from open source contributions to personal experiments."
                    </p>
                </div>
                <div id="click_to_load_rows" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8" data_fragment_merge_target="$has_next_page">
                    {projects.into_iter().map(|project| {
                        rsx!{ <ProjectArticle {..project.clone()} />}
                    })}
                </div>
                <ScrollCard intersect="@get('/projects/infinite_scroll')" />
            </div>
        </PageLayout>
    }
}
