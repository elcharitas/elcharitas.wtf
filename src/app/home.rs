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
            <section>
                <div class="grid grid-cols-1 xl:grid-cols-[minmax(0,1.6fr)_minmax(280px,0.4fr)] gap-6 xl:gap-10 w-full items-end">
                    <div class="space-y-3 max-w-4xl">
                        <div class="inline-flex items-center gap-1.5 text-[11px] uppercase tracking-[0.1em] font-semibold text-zinc-500">
                            <span class="w-1.5 h-1.5 rounded-full" style="background: var(--accent);"></span>
                            "Kehinde Jonathan Irhodia"
                        </div>
                        <h1 class="text-3xl md:text-5xl xl:text-6xl 2xl:text-7xl font-semibold text-zinc-950 leading-[0.94] tracking-[-0.045em]">
                            "I build software that holds up."
                        </h1>
                        <p class="text-zinc-600 max-w-2xl leading-relaxed text-sm md:text-base">
                            "Software and systems engineer working across Rust, Python, AI, and biodiagnostics."
                        </p>
                        <div class="flex flex-wrap gap-2">
                            <a href="/projects" class="btn-accent px-3 py-2 text-xs font-semibold rounded-md">
                                <i class="fas fa-cubes"></i>
                                "Explore projects"
                            </a>
                            <a href="/connect" class="btn-ghost px-3 py-2 text-xs font-semibold rounded-md">
                                <i class="far fa-message"></i>
                                "Let's talk"
                            </a>
                        </div>
                    </div>

                    <div class="space-y-2">
                        <div class="card-item rounded-lg p-3 space-y-2">
                            <div class="flex items-center justify-between">
                                <p class="text-[10px] uppercase tracking-[0.12em] font-semibold text-zinc-500">"Featured project"</p>
                                <i class="fas fa-bolt text-[10px]" style="color: var(--accent);"></i>
                            </div>
                            <div>
                                <div class="flex items-start justify-between gap-3">
                                    <p class="text-sm font-semibold text-zinc-950">{&featured.name}</p>
                                    <div class="flex items-center shrink-0">
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
                                <p class="mt-1 text-xs text-zinc-600 leading-relaxed line-clamp-2">{&featured.description}</p>
                            </div>
                            <div class="flex flex-wrap gap-1">
                                {featured.tags.iter().take(3).map(|tag| {
                                    <span class="text-[10px] px-1.5 py-0.5 rounded-full border border-zinc-200 text-zinc-600 bg-white">{tag}</span>
                                })}
                            </div>
                        </div>
                        <a href="/newsletter" class="group flex items-center justify-between gap-3 rounded-lg border border-zinc-200 bg-white/70 px-3 py-2 hover:border-orange-200 transition-colors">
                            <p class="text-xs text-zinc-700">"Weekly field notes from the build."</p>
                            <span class="icon-button shrink-0 group-hover:text-orange-600">
                                <i class="fas fa-arrow-right text-xs"></i>
                            </span>
                        </a>
                    </div>
                </div>
            </section>
        </PageLayout>
    }
}
