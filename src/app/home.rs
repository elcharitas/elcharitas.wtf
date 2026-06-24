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
            <section class="min-h-[70vh] flex items-center">
                <div class="grid grid-cols-1 lg:grid-cols-[1.1fr_0.9fr] gap-8 lg:gap-12 w-full items-center">
                    <div class="space-y-6">
                        <h2 class="text-3xl md:text-4xl font-semibold text-white">"I'm Jonathan Irhodia"</h2>
                        <h1 class="text-4xl md:text-6xl xl:text-7xl font-bold text-white leading-none">
                            "I build products"
                            <span class="text-zinc-400 typed-caret">"|"</span>
                        </h1>
                        <p class="text-zinc-300 max-w-2xl leading-relaxed text-base md:text-lg">
                            "Software engineer with a passion for building things. I work on Rust systems, write essays, and share what I learn while shipping."
                        </p>
                        <div class="flex flex-wrap gap-3 pt-2">
                            <a href="/projects" class="btn-accent px-5 py-3 text-sm font-semibold rounded-md">"My Portfolio"</a>
                            <a href="/connect" class="btn-ghost px-5 py-3 text-sm font-semibold rounded-md">"Contact Me"</a>
                        </div>
                    </div>

                    <div class="space-y-4">
                        <div class="border border-zinc-800/60 rounded-2xl p-5 bg-zinc-950/50 space-y-4">
                            <p class="text-xs uppercase tracking-[0.1em] text-zinc-500">"Featured Project"</p>
                            <div>
                                <div class="flex items-start justify-between gap-3">
                                    <p class="font-semibold text-zinc-100 text-base">{&featured.name}</p>
                                    <div class="flex items-center gap-2 shrink-0">
                                        <a href={&featured.url} target="_blank" rel="noopener noreferrer" class="social-link text-xs hover:text-zinc-300" aria_label="Source">
                                            <i class="fab fa-github text-base"></i>
                                        </a>
                                        {if !featured.homepage.is_empty() {
                                            rsx! {
                                                <a href={&featured.homepage} target="_blank" rel="noopener noreferrer" class="social-link text-xs hover:text-zinc-300" aria_label="Website">
                                                    <i class="fas fa-arrow-up-right-from-square text-sm"></i>
                                                </a>
                                            }
                                        } else {
                                            rsx! { <></> }
                                        }}
                                    </div>
                                </div>
                                <p class="mt-2 text-sm text-zinc-400 leading-relaxed">{&featured.description}</p>
                            </div>
                            <div class="flex flex-wrap gap-2">
                                {featured.tags.iter().map(|tag| {
                                    rsx! {
                                        <span class="text-xs px-2 py-0.5 rounded-full border border-zinc-700 text-zinc-400">{tag}</span>
                                    }
                                })}
                            </div>
                        </div>
                        <div class="border border-zinc-800 rounded-2xl p-5 bg-zinc-950/60">
                            <p class="text-xs uppercase tracking-[0.1em] text-zinc-500 mb-3">"Now"</p>
                            <p class="text-sm text-zinc-300 leading-relaxed">"Building and writing in public. Subscribe to the newsletter for weekly updates on what ships next."</p>
                        </div>
                    </div>
                </div>
            </section>
            <style>
                {r#"
                .typed-caret {
                    display: inline-block;
                    animation: caret-blink 1s steps(1, end) infinite;
                }

                @keyframes caret-blink {
                    0%, 45% { opacity: 1; }
                    46%, 100% { opacity: 0; }
                }
                "#}
            </style>
        </PageLayout>
    }
}
