use crate::components::PageLayout;
use axum::response::{Html, IntoResponse};
use comrak::{Options, markdown_to_html};
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
    pub month_index: usize,
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
            "fas fa-palette".to_string()
        } else if title_lower.contains("rust") || title_lower.contains("ngyn") {
            "fas fa-gears".to_string()
        } else if title_lower.contains("web3") || title_lower.contains("blockchain") {
            "fas fa-globe".to_string()
        } else if title_lower.contains("community") || title_lower.contains("hangout") {
            "fas fa-people-group".to_string()
        } else if title_lower.contains("bot") || title_lower.contains("telegram") {
            "fas fa-robot".to_string()
        } else if title_lower.contains("app") || title_lower.contains("mobile") {
            "fas fa-mobile-screen-button".to_string()
        } else if title_lower.contains("framework") || title_lower.contains("library") {
            "fas fa-screwdriver-wrench".to_string()
        } else if title_lower.contains("game") || title_lower.contains("engine") {
            "fas fa-gamepad".to_string()
        } else if title_lower.contains("php") || title_lower.contains("laravel") {
            "fas fa-lightbulb".to_string()
        } else if title_lower.contains("open source") || title_lower.contains("pull request") {
            "fas fa-code-branch".to_string()
        } else if title_lower.contains("business") || title_lower.contains("sold") {
            "fas fa-sack-dollar".to_string()
        } else if title_lower.contains("programming") || title_lower.contains("html") {
            "fas fa-seedling".to_string()
        } else {
            "fas fa-bolt".to_string()
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
            for (month_index, activity) in activities.into_iter().enumerate() {
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
                    month_index,
                    quarter,
                    icon: Adventure::get_icon(&activity),
                    // location: String::new(),
                    title: activity,
                });
            }
        }
    }

    // Sort by year and month (latest first)
    adventures.sort_by(|a, b| {
        let year_cmp = b.year.cmp(&a.year);
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
            let month_cmp = month_order(&b.month).cmp(&month_order(&a.month));
            if month_cmp == std::cmp::Ordering::Equal {
                b.month_index.cmp(&a.month_index)
            } else {
                month_cmp
            }
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

                    <div class="grid grid-cols-1 md:grid-cols-[100px_1fr] gap-4 md:gap-8">
                        <div class="md:pt-1">
                            <span class="text-4xl md:text-5xl font-bold text-white">{year.as_str()}</span>
                        </div>
                        <ul class="space-y-3">
                            {year_adventures.iter().map(|adventure| {
                                let is_major = adventure.title.len() > 60
                                    || adventure.title.contains("Framework")
                                    || adventure.title.contains("Joined")
                                    || adventure.title.contains("Started work");

                                <li class="flex items-start gap-3 group">
                                    <i class={format!("{} mt-1 text-sm shrink-0 text-zinc-400 group-hover:text-zinc-200 transition-colors", adventure.icon)}></i>
                                    <div class="space-y-0.5">
                                        <div class={format!("text-sm md:text-base leading-snug {}",
                                            if is_major { "text-zinc-100 font-medium" } else { "text-zinc-300" })}>
                                            <div _dangerously_set_inner_html={markdown_to_html(&adventure.title, &Options::default())} />
                                        </div>
                                        <p class="text-xs" style="color: var(--accent); opacity: 0.6;">{adventure.date.as_str()}" · "{adventure.quarter.as_str()}</p>
                                    </div>
                                </li>
                            })}
                        </ul>
                    </div>
                })}
            </div>
        </PageLayout>
    }
}
