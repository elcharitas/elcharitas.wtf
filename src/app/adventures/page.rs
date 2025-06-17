use crate::{components::PageLayout, shared::PageParams};
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

#[component]
pub fn AdventuresPage(_: &PageParams) -> Node {
    let adventures = parse_adventures_from_json();

    rsx! {
        <PageLayout title="Timeline">
            <div class="text-center mb-12">
                <h1 class="text-3xl md:text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-yellow-400 to-orange-400 mb-4">
                    "Journey Timeline"
                </h1>
                <p class="text-base text-zinc-300 max-w-2xl mx-auto">
                    "Over a decade of coding adventures, from first HTML tag to leading engineering teams."
                </p>

                <div class="flex justify-center gap-6 mt-6 text-sm">
                    <div class="text-yellow-400 font-semibold">{adventures.len() as u32}" Milestones"</div>
                    <div class="text-blue-400 font-semibold">"10+ Years"</div>
                    <div class="text-purple-400 font-semibold">"50+ Projects"</div>
                </div>
            </div>

            <div class="max-w-4xl mx-auto">
                <div class="relative">
                    <div class="absolute left-4 md:left-8 top-0 bottom-0 w-px bg-gradient-to-b from-yellow-400 via-blue-400 to-gray-600"></div>

                    <div class="space-y-6">
                        {adventures.iter().enumerate().map(|(index, adventure)| {
                            let is_major = adventure.title.len() > 60 ||
                                          adventure.title.contains("Framework") ||
                                          adventure.title.contains("Joined") ||
                                          adventure.title.contains("Started work");

                            rsx! {
                                <div class="relative flex items-start group">
                                    <div class="absolute left-4 md:left-8 w-3 h-3 rounded-full border-2 border-zinc-900 bg-gradient-to-r from-yellow-400 to-orange-400 transform -translate-x-1.5 z-10 group-hover:scale-125 transition-transform"></div>

                                    <div class="ml-12 md:ml-20 flex-1">
                                        <div class={format!("p-4 rounded-lg border border-zinc-800 bg-zinc-900/60 hover:border-zinc-600 hover:bg-zinc-900/80 transition-all duration-200 {}",
                                            if is_major { "border-l-4 border-l-yellow-500/50" } else { "" })}>

                                            <div class="flex items-center justify-between mb-2">
                                                <div class="flex items-center gap-2">
                                                    <span class="text-lg">{&adventure.icon}</span>
                                                    <div class="flex flex-col justify-between gap-2">
                                                        <div class="text-sm font-semibold text-yellow-400">
                                                            {&adventure.date}
                                                        </div>
                                                        <div class="text-xs text-zinc-500 flex items-center gap-1">
                                                            <i class="fas fa-calendar"></i>
                                                            {&adventure.quarter}
                                                        </div>
                                                    </div>
                                                </div>

                                                {when!(is_major =><div class="px-2 py-1 bg-yellow-500/20 text-yellow-400 text-xs rounded-full font-medium">
                                                    "Major"
                                                </div>)}
                                            </div>

                                            <p class="text-zinc-200 text-sm leading-relaxed group-hover:text-white transition-colors">
                                                {&adventure.title}
                                            </p>
                                        </div>

                                        {when!(index < adventures.len() - 1 => <div class="ml-2 mt-2 w-px h-4 bg-zinc-700"></div>)}
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                </div>
            </div>

            <div class="mt-12 text-center">
                <div class="inline-flex items-center gap-3 px-6 py-3 bg-zinc-900/60 border border-zinc-700 rounded-lg">
                    <span class="text-2xl">"🚀"</span>
                    <div>
                        <div class="text-white font-semibold">"The journey continues..."</div>
                        <div class="text-zinc-400 text-sm">"Building the future, one line of code at a time"</div>
                    </div>
                </div>
            </div>
        </PageLayout>
    }
}
