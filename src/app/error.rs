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
            <div class="py-6 md:py-10">
                <section class="glass-panel rounded-3xl p-8 md:p-10 max-w-3xl mx-auto">
                    <div class="text-center space-y-6">
                        <div class="relative mx-auto w-20 h-20 rounded-2xl border border-zinc-700 bg-zinc-900 flex items-center justify-center">
                            <i class="fas fa-compass text-2xl text-zinc-300"></i>
                        </div>
                        <div class="space-y-3">
                            <p class="text-xs uppercase tracking-[0.14em] text-zinc-500">"Error 404"</p>
                            <h2 class="text-4xl font-semibold text-zinc-100 tracking-tight">
                                "This route is off the map"
                            </h2>
                            <p class="text-lg text-zinc-300 max-w-xl mx-auto">
                                "The page could not be found, or it has moved to a different path in the site map."
                            </p>
                        </div>

                        <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 text-sm text-left">
                            <a href="/" class="border border-zinc-700 rounded-xl p-3 bg-zinc-900/60 hover:bg-zinc-900 transition-colors">
                                <p class="font-semibold text-zinc-100">"Home"</p>
                                <p class="text-zinc-400 text-xs mt-1">"Start from dashboard"</p>
                            </a>
                            <a href="/essays" class="border border-zinc-700 rounded-xl p-3 bg-zinc-900/60 hover:bg-zinc-900 transition-colors">
                                <p class="font-semibold text-zinc-100">"Essays"</p>
                                <p class="text-zinc-400 text-xs mt-1">"Read latest posts"</p>
                            </a>
                            <a href="/projects" class="border border-zinc-700 rounded-xl p-3 bg-zinc-900/60 hover:bg-zinc-900 transition-colors">
                                <p class="font-semibold text-zinc-100">"Projects"</p>
                                <p class="text-zinc-400 text-xs mt-1">"Browse work"</p>
                            </a>
                        </div>
                    </div>

                    <div class="flex justify-center space-x-4 mt-6">
                        <a
                            href="/"
                            class="transform transition-all duration-200 px-6 py-3 text-base font-medium rounded-lg text-white bg-zinc-700 hover:bg-zinc-600"
                        >
                            "Go Home"
                        </a>
                        <a
                            href="/adventures"
                            class="transform transition-all duration-200 px-6 py-3 text-base font-medium rounded-lg text-zinc-300 border border-zinc-600 hover:bg-zinc-700"
                        >
                            "View Timeline"
                        </a>
                    </div>
                </section>
            </div>
        </PageLayout>
    }
}
