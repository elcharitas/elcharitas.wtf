use crate::components::PageLayout;
use axum::response::{Html, IntoResponse};
use momenta::nodes::DefaultProps;
use momenta::prelude::*;

pub async fn resume_handler() -> impl IntoResponse {
    Html(ResumePage::render(&DefaultProps).to_string())
}

#[component]
pub fn ResumePage() -> Node {
    rsx! {
        <PageLayout title="Resume">
            <div class="max-w-4xl mx-auto px-4 text-white">
                <iframe class="min-h-screen w-full" src="/resume.docx" />
            </div>
        </PageLayout>
    }
}
