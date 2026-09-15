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
}

fn parse_adventures_from_json() -> Vec<Adventure> {
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
                            .next()
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
            <div class="py-10 md:py-14">
                <section class="page-heading">
                    <p class="eyebrow">"Archive"</p>
                    <h1 class="mt-3 text-4xl md:text-5xl font-semibold text-zinc-950">"Timeline"</h1>
                    <p class="mt-4 text-base text-zinc-600 max-w-3xl leading-relaxed">
                        "A decade of engineering milestones, product pivots, and experiments — in chronological order."
                    </p>
                </section>

                <div class="mt-10 space-y-12">
                    {years.iter().map(|year| {
                        let year_adventures: Vec<&Adventure> = adventures
                            .iter()
                            .filter(|a| &a.year == year)
                            .collect();

                        <div class="grid grid-cols-1 md:grid-cols-[90px_1fr] gap-4 md:gap-8">
                            <div class="md:pt-4">
                                <span class="text-2xl font-semibold text-zinc-950">{year.as_str()}</span>
                            </div>
                            <ul class="border-t border-zinc-900/15">
                                {year_adventures.iter().map(|adventure| {
                                    let is_major = adventure.title.len() > 60
                                        || adventure.title.contains("Framework")
                                        || adventure.title.contains("Joined")
                                        || adventure.title.contains("Started work");

                                    <li class="grid grid-cols-[84px_1fr] gap-4 py-4 border-b border-zinc-900/10">
                                        <p class="text-xs text-zinc-500">{adventure.date.as_str()}</p>
                                        <div>
                                            <div class={format!("text-sm leading-relaxed {}",
                                                if is_major { "text-zinc-950 font-medium" } else { "text-zinc-700" })}>
                                                <div _dangerously_set_inner_html={markdown_to_html(&adventure.title, &Options::default())} />
                                            </div>
                                            <p class="mt-1 text-[11px] uppercase tracking-[0.1em] text-zinc-400">{adventure.quarter.as_str()}</p>
                                        </div>
                                    </li>
                                })}
                            </ul>
                        </div>
                    })}
                </div>
            </div>
        </PageLayout>
    }
}
