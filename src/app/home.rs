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
            <div class="garden-home">
                <section class="garden-hero">
                    <div class="garden-hero-container">
                        <div class="garden-hero-copy">
                            <h1>"Hey, I'm Jonathan!"</h1>
                            <p class="garden-tagline">"Software engineer, biodiagnostics researcher, and open-source builder."</p>
                            <div class="garden-eras">
                                <h2>"A brief timeline"</h2>
                                <ul>
                                    <li><span>"2018–2020"</span><p>"Started building web applications and developer tools while studying biotechnology."</p></li>
                                    <li><span>"2021–2024"</span><p>"Moved into professional software engineering, research internships, and the first generation of Rust open-source work."</p></li>
                                    <li><span>"2025–now"</span><p>"Building systems software and research tools while working on affordable diagnostics, microfluidics, and applied AI."</p></li>
                                </ul>
                            </div>
                            <p class="garden-hero-note">"I care about reliable tools, practical research, and sharing the path from an idea to something useful."</p>
                        </div>
                        <aside class="garden-hero-profile">
                            <div class="garden-avatar-wrap"><img src="https://github.com/elcharitas.png?size=192" alt="Kehinde J. Irhodia" /></div>
                            <p>"A systems notebook, lab journal, and archive for work made in public."</p>
                        </aside>
                    </div>
                </section>

                <section class="garden-index-section">
                    <header class="garden-heading">
                        <h2>"Latest writing"</h2>
                        <a href="/essays" class="garden-button">"All essays"</a>
                    </header>
                    <div class="garden-post-list">
                        <a href="/essays/taking-the-bull-by-the-horn-with-hashnode-headless" class="garden-post-row">
                            <span class="garden-post-icon">"✦"</span>
                            <span><strong>"Taking the Bull by the Horn with Hashnode Headless"</strong><time>"Oct 31, 2023"</time></span>
                            <span class="garden-post-topic">"web development"</span>
                        </a>
                        <a href="/essays/the-chatgpt-authorship-dilemma-is-the-ai-model-a-thief" class="garden-post-row">
                            <span class="garden-post-icon">"✦"</span>
                            <span><strong>"The ChatGPT Authorship Dilemma: Is the AI Model a Thief?"</strong><time>"Apr 4, 2023"</time></span>
                            <span class="garden-post-topic">"AI"</span>
                        </a>
                        <a href="/essays/improving-the-performance-of-python-backends" class="garden-post-row">
                            <span class="garden-post-icon">"✦"</span>
                            <span><strong>"Improving the Performance of Python Backends"</strong><time>"Mar 10, 2023"</time></span>
                            <span class="garden-post-topic">"systems"</span>
                        </a>
                    </div>
                </section>

                <section class="garden-index-section">
                    <header class="garden-heading">
                        <h2>"Current directions"</h2>
                        <a href="/publications" class="garden-button">"Research"</a>
                    </header>
                    <div class="garden-cards garden-cards-half">
                        <a href="/publications" class="garden-card garden-card-highlight">
                            <strong>"Biodiagnostics"</strong>
                            <p>"Aptamer biosensors, microfluidic devices, and affordable point-of-care diagnostics."</p>
                        </a>
                        <a href="/projects" class="garden-card garden-card-highlight">
                            <strong>"Open-source systems"</strong>
                            <p>"Rust frameworks, data pipelines, and tools that make complex work more dependable."</p>
                        </a>
                    </div>
                </section>

                <section class="garden-index-section">
                    <header class="garden-heading">
                        <h2>"Projects"</h2>
                        <a href="/projects" class="garden-button">"All projects"</a>
                    </header>
                    <div class="garden-cards">
                        <article class="garden-card">
                            <time>"2025"</time>
                            <a href={&featured.url} target="_blank" rel="noopener noreferrer">{&featured.name}</a>
                            <p>{&featured.description}</p>
                            <div><a href={&featured.url} target="_blank" rel="noopener noreferrer">"Source ↗"</a>{if !featured.homepage.is_empty() { rsx! { <a href={&featured.homepage} target="_blank" rel="noopener noreferrer">"Demo ↗"</a> } } else { rsx! { <></> } }}</div>
                        </article>
                        <article class="garden-card">
                            <time>"2025"</time>
                            <a href="https://github.com/elcharitas/bioinformatics_rs" target="_blank" rel="noopener noreferrer">"bioinformatics_rs"</a>
                            <p>"High-performance sequence-alignment algorithms and bioinformatics experiments in Rust."</p>
                            <div><a href="https://github.com/elcharitas/bioinformatics_rs" target="_blank" rel="noopener noreferrer">"Source ↗"</a></div>
                        </article>
                    </div>
                </section>
            </div>
        </PageLayout>
    }
}
