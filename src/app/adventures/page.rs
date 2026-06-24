use crate::components::PageLayout;
use axum::response::{Html, IntoResponse};
use momenta::nodes::DefaultProps;
use momenta::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct AdventuresData {
    #[serde(flatten)]
    pub years: HashMap<String, HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone)]
pub struct Adventure {
    pub date: String,
    pub month: String,
    pub year: String,
    pub quarter: String,
    pub title: String,
    pub icon: String,
    // pub location: String,
}

impl Adventure {
    fn get_quarter(month: &str) -> String {
        match month.to_lowercase().as_str() {
            "january" | "february" | "march" => "Q1".to_string(),
            "april" | "may" | "june" => "Q2".to_string(),
            "july" | "august" | "september" => "Q3".to_string(),
            "october" | "november" | "december" => "Q4".to_string(),
            _ => "Q1".to_string(),
        }
    }

    fn get_icon(title: &str) -> String {
        let title_lower = title.to_lowercase();
        if title_lower.contains("chakra") || title_lower.contains("ui") {
            "🎨".to_string()
        } else if title_lower.contains("rust") || title_lower.contains("ngyn") {
            "🦀".to_string()
        } else if title_lower.contains("web3") || title_lower.contains("blockchain") {
            "🌐".to_string()
        } else if title_lower.contains("community") || title_lower.contains("hangout") {
            "🤝".to_string()
        } else if title_lower.contains("bot") || title_lower.contains("telegram") {
            "🤖".to_string()
        } else if title_lower.contains("app") || title_lower.contains("mobile") {
            "📱".to_string()
        } else if title_lower.contains("framework") || title_lower.contains("library") {
            "🔧".to_string()
        } else if title_lower.contains("game") || title_lower.contains("engine") {
            "🎮".to_string()
        } else if title_lower.contains("php") || title_lower.contains("laravel") {
            "💡".to_string()
        } else if title_lower.contains("open source") || title_lower.contains("pull request") {
            "🔓".to_string()
        } else if title_lower.contains("business") || title_lower.contains("sold") {
            "💰".to_string()
        } else if title_lower.contains("programming") || title_lower.contains("html") {
            "🌱".to_string()
        } else {
            "⚡".to_string()
        }
    }
}

fn parse_adventures_from_json() -> Vec<Adventure> {
    // This would normally read from a file, but for demo purposes, we'll use the embedded JSON
    let json_data = include_str!("./adventures.json");

    let data: AdventuresData = serde_json::from_str(json_data).unwrap_or_default();
    let mut adventures = Vec::new();

    for (year, months) in data.years {
        for (month, activities) in months {
            for activity in activities {
                let quarter = Adventure::get_quarter(&month);
                adventures.push(Adventure {
                    date: format!(
                        "{} {}",
                        month
                            .chars()
                            .nth(0)
                            .unwrap()
                            .to_uppercase()
                            .collect::<String>()
                            + &month[1..],
                        year
                    ),
                    month: month.clone(),
                    year: year.clone(),
                    quarter,
                    icon: Adventure::get_icon(&activity),
                    // location: String::new(),
                    title: activity,
                });
            }
        }
    }

    // Sort by year and month
    adventures.sort_by(|a, b| {
        let year_cmp = a.year.cmp(&b.year);
        if year_cmp == std::cmp::Ordering::Equal {
            let month_order = |m: &str| match m {
                "january" => 1,
                "february" => 2,
                "march" => 3,
                "april" => 4,
                "may" => 5,
                "june" => 6,
                "july" => 7,
                "august" => 8,
                "september" => 9,
                "october" => 10,
                "november" => 11,
                "december" => 12,
                _ => 0,
            };
            month_order(&a.month).cmp(&month_order(&b.month))
        } else {
            year_cmp
        }
    });

    adventures
}

pub async fn adventures_handler() -> impl IntoResponse {
    Html(AdventuresPage::render(&DefaultProps).to_string())
}

#[component]
pub fn AdventuresPage() -> Node {
    let adventures = parse_adventures_from_json();

    // Collect unique years sorted descending
    let mut years: Vec<String> = adventures
        .iter()
        .map(|a| a.year.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    years.sort_by(|a, b| b.cmp(a));

    rsx! {
        <PageLayout title="Timeline">
            <section class="space-y-3 mb-10">
                <h1 class="text-4xl md:text-5xl font-semibold text-white">"Timeline"</h1>
                <div class="section-rule"></div>
                <p class="text-base text-zinc-300 max-w-3xl">
                    "A decade of engineering milestones, product pivots, and experiments — in chronological order."
                </p>
            </section>

            <div class="space-y-14">
                {years.iter().map(|year| {
                    let year_adventures: Vec<&Adventure> = adventures
                        .iter()
                        .filter(|a| &a.year == year)
                        .collect();

                    rsx! {
                        <div class="grid grid-cols-1 md:grid-cols-[100px_1fr] gap-4 md:gap-8">
                            <div class="md:pt-1">
                                <span class="text-4xl md:text-5xl font-bold" style="color: var(--accent); opacity: 0.35;">{year.as_str()}</span>
                            </div>
                            <ul class="space-y-3">
                                {year_adventures.iter().map(|adventure| {
                                    let is_major = adventure.title.len() > 60
                                        || adventure.title.contains("Framework")
                                        || adventure.title.contains("Joined")
                                        || adventure.title.contains("Started work");

                                    rsx! {
                                        <li class="flex items-start gap-3 group">
                                            <span class="mt-1 text-base shrink-0">{adventure.icon.as_str()}</span>
                                            <div class="space-y-0.5">
                                                <p class={format!("text-sm md:text-base leading-snug {}",
                                                    if is_major { "text-zinc-100 font-medium" } else { "text-zinc-300" })}>
                                                    {adventure.title.as_str()}
                                                </p>
                                                <p class="text-xs" style="color: var(--accent); opacity: 0.6;">{adventure.date.as_str()}" · "{adventure.quarter.as_str()}</p>
                                            </div>
                                        </li>
                                    }
                                })}
                            </ul>
                        </div>
                    }
                })}
            </div>
        </PageLayout>
    }
}
