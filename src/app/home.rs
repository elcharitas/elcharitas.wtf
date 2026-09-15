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
            <div class="py-12 md:py-16 lg:py-20">
                <section class="grid grid-cols-1 lg:grid-cols-[minmax(0,1.45fr)_minmax(260px,0.55fr)] gap-12 lg:gap-20 items-end">
                    <div class="max-w-4xl">
                        <p class="eyebrow">"Kehinde Jonathan Irhodia"</p>
                        <h1 class="mt-5 text-4xl md:text-6xl xl:text-7xl font-semibold text-zinc-950 leading-[0.98] tracking-[-0.05em]">
                            "Engineering reliable systems. Researching better diagnostics."
                        </h1>
                        <p class="mt-6 text-base md:text-lg text-zinc-600 max-w-2xl leading-relaxed">
                            "I work across software, AI, and biodiagnostics, turning complex ideas into practical tools."
                        </p>
                        <div class="mt-7 flex flex-wrap items-center gap-5">
                            <a href="/projects" class="primary-link">"View selected work ↗"</a>
                            <a href="/resume" class="text-link">"Read my résumé →"</a>
                        </div>
                    </div>

                    <aside class="border-l border-zinc-900/15 pl-6 md:pl-8">
                        <p class="eyebrow">"Areas of focus"</p>
                        <div class="mt-4">
                            <div class="focus-row">
                                <p class="focus-title">"Engineering"</p>
                                <p class="focus-detail">"Rust, Python, infrastructure"</p>
                            </div>
                            <div class="focus-row">
                                <p class="focus-title">"Research"</p>
                                <p class="focus-detail">"Biodiagnostics, microfluidics"</p>
                            </div>
                            <div class="focus-row">
                                <p class="focus-title">"Writing"</p>
                                <p class="focus-detail">"Systems, products, open source"</p>
                            </div>
                        </div>
                    </aside>
                </section>

                <section class="mt-16 md:mt-20 pt-6 border-t border-zinc-900/15">
                    <div class="flex items-baseline justify-between gap-4">
                        <p class="eyebrow">"Selected work"</p>
                        <a href="/projects" class="text-link text-xs">"All projects →"</a>
                    </div>
                    <div class="mt-5 grid grid-cols-1 md:grid-cols-[minmax(0,1fr)_240px] gap-8 md:gap-12">
                        <div>
                            <div class="flex items-start justify-between gap-6">
                                <div>
                                    <h2 class="text-xl font-semibold text-zinc-950">{&featured.name}</h2>
                                    <p class="mt-2 text-sm text-zinc-600 leading-relaxed max-w-3xl">{&featured.description}</p>
                                </div>
                                <div class="flex items-center gap-4 shrink-0 text-xs">
                                    <a href={&featured.url} target="_blank" rel="noopener noreferrer" class="text-link">"Source ↗"</a>
                                    {if !featured.homepage.is_empty() {
                                        rsx! {
                                            <a href={&featured.homepage} target="_blank" rel="noopener noreferrer" class="text-link">"Visit ↗"</a>
                                        }
                                    } else {
                                        rsx! { <></> }
                                    }}
                                </div>
                            </div>
                            <div class="mt-4 flex flex-wrap gap-x-4 gap-y-1">
                                {featured.tags.iter().take(4).map(|tag| {
                                    <span class="meta-label">{tag}</span>
                                })}
                            </div>
                        </div>
                        <a href="/newsletter" class="group border-t md:border-t-0 md:border-l border-zinc-900/15 pt-5 md:pt-0 md:pl-6">
                            <span class="eyebrow">"Field notes"</span>
                            <span class="mt-2 block text-sm text-zinc-700 leading-relaxed group-hover:text-zinc-950 transition-colors">
                                "Weekly notes on engineering, research, and the work in between. →"
                            </span>
                        </a>
                    </div>
                </section>
            </div>
        </PageLayout>
    }
}
