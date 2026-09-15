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
            <div class="py-16 md:py-20">
                <section class="max-w-2xl">
                    <p class="eyebrow">"404"</p>
                    <h1 class="mt-3 text-4xl md:text-5xl font-semibold text-zinc-950">"Page not found"</h1>
                    <p class="mt-4 text-base text-zinc-600">
                        "This page doesn't exist or has moved. Try one of these instead."
                    </p>
                </section>

                <div class="mt-7 flex gap-5 flex-wrap">
                    <a href="/" class="primary-link">"Go home"</a>
                    <a href="/essays" class="text-link">"Browse essays →"</a>
                    <a href="/projects" class="text-link">"View projects →"</a>
                </div>
            </div>
        </PageLayout>
    }
}
