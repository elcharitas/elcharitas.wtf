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
            <section class="py-4 md:py-8 space-y-12">

                // Header
                <div class="space-y-3">
                    <h1 class="text-4xl md:text-5xl font-semibold text-white">{&data.header.name}</h1>
                    <div class="section-rule"></div>
                    <p class="text-zinc-300 text-lg leading-relaxed">{&data.header.bio}</p>
                    <div class="flex flex-wrap gap-3 pt-1">
                        {data.header.links.iter().map(|link| {
                            let cls = format!(
                                "px-4 py-2 text-sm font-semibold rounded-md {}",
                                if link.style == "accent" { "btn-accent" } else { "btn-ghost" }
                            );
                            if link.external {
                                <a href={&link.href} class={&cls} target="_blank" rel="noopener noreferrer">{&link.label}</a>
                            } else {
                                <a href={&link.href} class={&cls}>{&link.label}</a>
                            }
                        })}
                    </div>
                </div>

                // Experience
                <div class="space-y-6">
                    <h2 class="text-xl font-semibold text-white uppercase tracking-[0.1em]">"Experience"</h2>
                    <div class="space-y-8">
                        {data.experience.iter().map(|entry| {
                            <div class="grid grid-cols-1 sm:grid-cols-[140px_1fr] gap-2 sm:gap-6">
                                <p class="text-sm text-zinc-500 sm:pt-1">{&entry.period}</p>
                                <div>
                                    <p class="font-semibold text-zinc-100">{&entry.title}</p>
                                    <p class="text-sm" style="color: var(--accent);">{&entry.entry_type}</p>
                                    <p class="mt-2 text-sm text-zinc-400 leading-relaxed">{&entry.description}</p>
                                </div>
                            </div>
                        })}
                    </div>
                </div>

                // Skills
                <div class="space-y-4">
                    <h2 class="text-xl font-semibold text-white uppercase tracking-[0.1em]">"Skills"</h2>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                        {data.skills.iter().map(|skill| {
                            <div class="card-item rounded-xl p-4 space-y-2">
                                <p class="text-xs uppercase tracking-[0.1em]" style="color: var(--accent);">{&skill.category}</p>
                                <p class="text-sm text-zinc-300">{&skill.items}</p>
                            </div>
                        })}
                    </div>
                </div>

                <div class="space-y-4">
                    <h2 class="text-xl font-semibold text-white uppercase tracking-[0.1em]">"Research Interests"</h2>
                    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                        {data.interests.iter().map(|interest| {
                            <div class="card-item rounded-xl p-4 space-y-2">
                                <p class="text-xs uppercase tracking-[0.1em]" style="color: var(--accent);">"Publication focus"</p>
                                <p class="text-sm text-zinc-300">{interest}</p>
                            </div>
                        })}
                    </div>
                </div>

                // Education
                <div class="space-y-4">
                    <h2 class="text-xl font-semibold text-white uppercase tracking-[0.1em]">"Education"</h2>
                    <div class="space-y-6">
                        {data.education.iter().map(|entry| {
                            <div class="grid grid-cols-1 sm:grid-cols-[140px_1fr] gap-2 sm:gap-6">
                                <p class="text-sm text-zinc-500">{&entry.period}</p>
                                <div>
                                    <p class="font-semibold text-zinc-100">{&entry.degree}</p>
                                    <p class="text-sm text-zinc-400">{&entry.school}</p>
                                </div>
                            </div>
                        })}
                    </div>
                </div>

            </section>
        </PageLayout>
    }
}
