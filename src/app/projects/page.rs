use crate::{
    components::{article::ProjectArticle, PageLayout},
    requests::get_all_projects,
    shared::{PageLoader, Project},
};
use momenta::prelude::*;

pub struct ProjectsProps {
    projects: Vec<Project>,
}

impl PageLoader for ProjectsProps {
    async fn load(_: &mut ngyn::shared::server::NgynContext<'_>) -> Self {
        let projects = get_all_projects(1).await.unwrap();

        Self { projects }
    }
}

#[component]
pub fn ProjectsPage(ProjectsProps { projects }: &ProjectsProps) -> Node {
    rsx! {
        <PageLayout title="Projects">
            <div class="container mx-auto px-4 py-12">
                <h1 class="text-4xl font-bold text-zinc-50 mb-8">"Projects 🚀"</h1>
                <p class="text-lg text-zinc-200 mb-12">
                    "Here are some of the projects I've worked on, from open source contributions to personal experiments."
                </p>
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
