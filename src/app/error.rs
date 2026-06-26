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
            <div class="py-4 md:py-8 space-y-8">
                <section class="space-y-4">
                    <div class="flex items-center gap-4">
                        <div class="w-12 h-12 rounded-xl border border-zinc-700 bg-zinc-900 flex items-center justify-center shrink-0">
                            <i class="fas fa-compass text-lg text-zinc-300"></i>
                        </div>
                        <h1 class="text-4xl md:text-5xl font-semibold text-white">"Page Not Found"</h1>
                    </div>
                    <div class="section-rule"></div>
                    <p class="text-base text-zinc-300 max-w-3xl">
                        "Error 404 — the page could not be found, or it has moved to a different path. Here are some places to get back on track."
                    </p>
                </section>

                <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                    <a href="/" class="card-item soft-lift rounded-2xl p-5 block">
                        <div class="space-y-1">
                            <p class="font-semibold text-zinc-100">"Home"</p>
                            <p class="text-zinc-400 text-sm">"Start from the beginning"</p>
                        </div>
                    </a>
                    <a href="/essays" class="card-item soft-lift rounded-2xl p-5 block">
                        <div class="space-y-1">
                            <p class="font-semibold text-zinc-100">"Essays"</p>
                            <p class="text-zinc-400 text-sm">"Read the latest posts"</p>
                        </div>
                    </a>
                    <a href="/projects" class="card-item soft-lift rounded-2xl p-5 block">
                        <div class="space-y-1">
                            <p class="font-semibold text-zinc-100">"Projects"</p>
                            <p class="text-zinc-400 text-sm">"Browse open-source work"</p>
                        </div>
                    </a>
                </div>

                <div class="flex gap-3 flex-wrap">
                    <a href="/" class="btn-accent px-6 py-2.5 text-sm font-medium rounded-lg">"Go Home"</a>
                    <a href="/adventures" class="btn-ghost px-6 py-2.5 text-sm font-medium rounded-lg">"View Timeline"</a>
                </div>
            </div>
        </PageLayout>
    }
}
