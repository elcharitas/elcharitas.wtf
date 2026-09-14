use crate::components::PageLayout;
use axum::response::{Html, IntoResponse};
use momenta::nodes::DefaultProps;
use momenta::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct FeaturedProject {
    name: String,
    description: String,
    url: String,
    homepage: String,
    tags: Vec<String>,
}

fn load_featured() -> FeaturedProject {
    let json = include_str!("./featured.json");
    serde_json::from_str(json).expect("featured.json is valid")
}

pub async fn home_handler() -> impl IntoResponse {
    Html(HomePage::render(&DefaultProps).to_string())
}

#[component]
pub fn HomePage() -> Node {
    let featured = load_featured();

    rsx! {
        <PageLayout title="Home">
            <section class="min-h-[72vh] flex items-center py-8">
                <div class="grid grid-cols-1 xl:grid-cols-[minmax(0,1.45fr)_minmax(360px,0.55fr)] gap-12 xl:gap-20 w-full items-end">
                    <div class="space-y-8 max-w-5xl">
                        <div class="inline-flex items-center gap-2 text-sm font-medium text-zinc-600">
                            <span class="w-2 h-2 rounded-full" style="background: var(--accent);"></span>
                            "Jonathan Irhodia · Software engineer"
                        </div>
                        <h1 class="text-5xl md:text-7xl xl:text-8xl 2xl:text-9xl font-semibold text-zinc-950 leading-[0.92] tracking-[-0.055em]">
                            "I build useful"
                            <br />
                            "things."
                        </h1>
                        <p class="text-zinc-600 max-w-3xl leading-relaxed text-base md:text-xl">
                            "Software engineer with a passion for building things. I work on Rust systems, write essays, and publish around biodiagnostics, AI, and microfluidics."
                        </p>
                        <div class="flex flex-wrap gap-3 pt-2">
                            <a href="/projects" class="btn-accent px-5 py-3 text-sm font-semibold rounded-xl">
                                <i class="fas fa-cubes"></i>
                                "Explore projects"
                            </a>
                            <a href="/connect" class="btn-ghost px-5 py-3 text-sm font-semibold rounded-xl">
                                <i class="far fa-message"></i>
                                "Get in touch"
                            </a>
                        </div>
                    </div>

                    <div class="space-y-3">
                        <div class="card-item rounded-2xl p-6 space-y-5">
                            <div class="flex items-center justify-between">
                                <p class="text-xs uppercase tracking-[0.12em] font-semibold text-zinc-500">"Featured project"</p>
                                <span class="inline-flex items-center justify-center w-8 h-8 rounded-lg" style="background: var(--accent-dim); color: var(--accent);">
                                    <i class="fas fa-bolt text-xs"></i>
                                </span>
                            </div>
                            <div>
                                <div class="flex items-start justify-between gap-3">
                                    <p class="font-semibold text-zinc-950 text-lg">{&featured.name}</p>
                                    <div class="flex items-center gap-2 shrink-0">
                                        <a href={&featured.url} target="_blank" rel="noopener noreferrer" class="icon-button" aria_label="Source" title="View source">
                                            <i class="fab fa-github text-base"></i>
                                        </a>
                                        {if !featured.homepage.is_empty() {
                                            rsx! {
                                                <a href={&featured.homepage} target="_blank" rel="noopener noreferrer" class="icon-button" aria_label="Website" title="Open website">
                                                    <i class="fas fa-arrow-up-right-from-square text-sm"></i>
                                                </a>
                                            }
                                        } else {
                                            rsx! { <></> }
                                        }}
                                    </div>
                                </div>
                                <p class="mt-2 text-sm text-zinc-600 leading-relaxed">{&featured.description}</p>
                            </div>
                            <div class="flex flex-wrap gap-2">
                                {featured.tags.iter().map(|tag| {
                                    <span class="text-xs px-2.5 py-1 rounded-full border border-zinc-200 text-zinc-600 bg-white">{tag}</span>
                                })}
                            </div>
                        </div>
                        <a href="/newsletter" class="group flex items-center justify-between gap-4 rounded-2xl border border-zinc-200 bg-white/70 p-5 hover:border-orange-200 transition-colors">
                            <div>
                                <p class="text-xs uppercase tracking-[0.12em] font-semibold text-zinc-500 mb-2">"Now"</p>
                                <p class="text-sm text-zinc-700 leading-relaxed">"Building and writing in public. Get the weekly field note."</p>
                            </div>
                            <span class="icon-button shrink-0 group-hover:text-orange-600">
                                <i class="fas fa-arrow-right text-sm"></i>
                            </span>
                        </a>
                    </div>
                </div>
            </section>
        </PageLayout>
    }
}
