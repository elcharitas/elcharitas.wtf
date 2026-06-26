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
            <div class="py-4 md:py-8 space-y-6">
                <section class="space-y-3">
                    <h1 class="text-4xl md:text-5xl font-semibold text-white">"Page Not Found"</h1>
                    <div class="section-rule"></div>
                    <p class="text-base text-zinc-400 max-w-2xl">
                        "This page doesn't exist or has moved. Try one of these instead."
                    </p>
                </section>

                <div class="flex gap-3 flex-wrap">
                    <a href="/" class="btn-accent px-6 py-2.5 text-sm font-medium rounded-lg">"Go Home"</a>
                    <a href="/essays" class="btn-ghost px-6 py-2.5 text-sm font-medium rounded-lg">"Essays"</a>
                    <a href="/projects" class="btn-ghost px-6 py-2.5 text-sm font-medium rounded-lg">"Projects"</a>
                </div>
            </div>
        </PageLayout>
    }
}
