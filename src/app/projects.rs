use std::cell::RefCell;

use crate::{
    components::{article::ProjectArticle, PageLayout},
    requests::get_all_projects,
    shared::{PageLoader, Project},
};
use momenta::prelude::*;

thread_local! {
    static PROJECTS: RefCell<Vec<Project>> = RefCell::new(Vec::new());
}

pub struct ProjectsProps {
    projects: Vec<Project>,
}

impl PageLoader for ProjectsProps {
    async fn load(_: &mut ngyn::shared::server::NgynContext<'_>) -> Self {
        let mut projects = PROJECTS.with_borrow(|projects| projects.clone());

        if projects.is_empty() {
            projects = get_all_projects(1).await.unwrap_or_default();
            PROJECTS.with_borrow_mut(|stored| {
                *stored = projects.clone();
            });
        }

        Self { projects }
    }
}

#[component]
pub fn ProjectsPage(ProjectsProps { projects }: &ProjectsProps) -> Node {
    rsx! {
        <PageLayout title="Projects">
            <div class="container mx-auto px-4 py-12">
                <div class="text-center mb-12">
                    <h1 class="text-3xl md:text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-yellow-400 to-orange-400 mb-4">
                        "Projects"
                    </h1>
                    <p class="text-base text-zinc-300 max-w-2xl mx-auto">
                        "Here are some of the projects I've worked on, from open source contributions to personal experiments."
                    </p>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {projects.iter().map(|project| {
                        let project = project.clone();
                        rsx!{ <ProjectArticle {..project} />}
                    })}
                </div>
            </div>
        </PageLayout>
    }
}
