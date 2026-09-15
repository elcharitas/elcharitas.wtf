use crate::components::PageLayout;
use axum::response::{Html, IntoResponse};
use momenta::nodes::DefaultProps;
use momenta::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ResumeLink {
    pub label: String,
    pub href: String,
    pub style: String,
    #[serde(default)]
    pub external: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ResumeHeader {
    pub name: String,
    pub bio: String,
    pub links: Vec<ResumeLink>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ExperienceEntry {
    pub period: String,
    pub title: String,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SkillEntry {
    pub category: String,
    pub items: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct EducationEntry {
    pub period: String,
    pub degree: String,
    pub school: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ResumeData {
    pub header: ResumeHeader,
    pub experience: Vec<ExperienceEntry>,
    pub skills: Vec<SkillEntry>,
    pub interests: Vec<String>,
    pub education: Vec<EducationEntry>,
}

fn load_resume() -> ResumeData {
    let json = include_str!("./resume.json");
    serde_json::from_str(json).expect("resume.json is valid")
}

pub async fn resume_handler() -> impl IntoResponse {
    Html(ResumePage::render(&DefaultProps).to_string())
}

#[component]
pub fn ResumePage() -> Node {
    let data = load_resume();

    rsx! {
        <PageLayout title="Resume">
            <section class="py-10 md:py-14 space-y-12">

                // Header
                <div class="page-heading">
                    <p class="eyebrow">"Résumé"</p>
                    <h1 class="mt-3 text-4xl md:text-5xl font-semibold text-zinc-950">{&data.header.name}</h1>
                    <p class="mt-4 text-zinc-600 text-base leading-relaxed max-w-4xl">{&data.header.bio}</p>
                    <div class="mt-6 flex flex-wrap gap-x-5 gap-y-2">
                        {data.header.links.iter().map(|link| {
                            let cls = if link.style == "accent" { "primary-link" } else { "text-link text-sm" };
                            if link.external {
                                <a href={&link.href} class={cls} target="_blank" rel="noopener noreferrer">{&link.label}</a>
                            } else {
                                <a href={&link.href} class={cls}>{&link.label}</a>
                            }
                        })}
                    </div>
                </div>

                // Experience
                <div>
                    <h2 class="eyebrow">"Experience"</h2>
                    <div class="mt-4 border-t border-zinc-900/15">
                        {data.experience.iter().map(|entry| {
                            <div class="grid grid-cols-1 sm:grid-cols-[140px_1fr] gap-2 sm:gap-8 py-5 border-b border-zinc-900/10">
                                <p class="text-xs text-zinc-500 sm:pt-1">{&entry.period}</p>
                                <div>
                                    <p class="font-semibold text-zinc-900">{&entry.title}</p>
                                    <p class="mt-1 text-xs text-zinc-500">{&entry.entry_type}</p>
                                    <p class="mt-3 text-sm text-zinc-600 leading-relaxed max-w-4xl">{&entry.description}</p>
                                </div>
                            </div>
                        })}
                    </div>
                </div>

                // Skills
                <div>
                    <h2 class="eyebrow">"Skills"</h2>
                    <div class="mt-4 grid grid-cols-1 sm:grid-cols-2 gap-x-8">
                        {data.skills.iter().map(|skill| {
                            <div class="border-t border-zinc-900/15 py-4">
                                <p class="text-sm font-medium text-zinc-900">{&skill.category}</p>
                                <p class="mt-2 text-sm text-zinc-600 leading-relaxed">{&skill.items}</p>
                            </div>
                        })}
                    </div>
                </div>

                <div>
                    <h2 class="eyebrow">"Research interests"</h2>
                    <div class="mt-4 grid grid-cols-1 sm:grid-cols-3 gap-x-8">
                        {data.interests.iter().map(|interest| {
                            <div class="border-t border-zinc-900/15 py-4">
                                <p class="text-sm text-zinc-700">{interest}</p>
                            </div>
                        })}
                    </div>
                </div>

                // Education
                <div>
                    <h2 class="eyebrow">"Education"</h2>
                    <div class="mt-4 border-t border-zinc-900/15">
                        {data.education.iter().map(|entry| {
                            <div class="grid grid-cols-1 sm:grid-cols-[140px_1fr] gap-2 sm:gap-8 py-5 border-b border-zinc-900/10">
                                <p class="text-xs text-zinc-500">{&entry.period}</p>
                                <div>
                                    <p class="font-semibold text-zinc-900">{&entry.degree}</p>
                                    <p class="text-sm text-zinc-600">{&entry.school}</p>
                                </div>
                            </div>
                        })}
                    </div>
                </div>

            </section>
        </PageLayout>
    }
}
