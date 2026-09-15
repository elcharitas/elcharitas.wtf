use crate::components::PageLayout;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use momenta::nodes::DefaultProps;
use momenta::prelude::*;

pub async fn error_handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html(ErrorPage::render(&DefaultProps).to_string()),
    )
}

#[component]
pub fn ErrorPage() -> Node {
    rsx! {
        <PageLayout title="Page Not Found - 404">
            <div class="py-0 md:py-2 space-y-4">
                <section class="space-y-2">
                    <h1 class="text-2xl md:text-3xl font-semibold text-zinc-950">"Page Not Found"</h1>
                    <div class="section-rule"></div>
                    <p class="text-sm md:text-base text-zinc-600 max-w-2xl">
                        "This page doesn't exist or has moved. Try one of these instead."
                    </p>
                </section>

                <div class="flex gap-2 flex-wrap">
                    <a href="/" class="btn-accent px-3 py-2 text-xs font-medium rounded-md"><i class="fas fa-house"></i>"Go home"</a>
                    <a href="/essays" class="btn-ghost px-3 py-2 text-xs font-medium rounded-md"><i class="far fa-pen-to-square"></i>"Essays"</a>
                    <a href="/projects" class="btn-ghost px-3 py-2 text-xs font-medium rounded-md"><i class="fas fa-cubes"></i>"Projects"</a>
                </div>
            </div>
        </PageLayout>
    }
}
