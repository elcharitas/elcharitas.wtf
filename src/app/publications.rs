use crate::components::PageLayout;
use crate::shared::get_env;
use axum::response::{Html, IntoResponse};
use momenta::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicationEntry {
    pub title: String,
    pub venue: Option<String>,
    pub published_on: Option<String>,
    pub work_type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PublicationsProps {
    pub orcid_id: Option<String>,
    pub orcid_url: Option<String>,
    pub publications: Vec<PublicationEntry>,
    pub notice: Option<String>,
}

impl PublicationsProps {
    async fn load() -> Self {
        let orcid_id = normalize_orcid_id(&get_env("ORCID_ID"));
        if orcid_id.is_empty() {
            return Self {
                notice: Some(
                    "Set your publication profile ID in your environment to sync automatically."
                        .to_string(),
                ),
                ..Default::default()
            };
        }

        let orcid_url = format!("https://orcid.org/{orcid_id}");
        match fetch_publications(&orcid_id).await {
            Ok(publications) => Self {
                orcid_id: Some(orcid_id),
                orcid_url: Some(orcid_url),
                publications,
                notice: None,
            },
            Err(error) => Self {
                orcid_id: Some(orcid_id),
                orcid_url: Some(orcid_url),
                publications: Vec::new(),
                notice: Some(error),
            },
        }
    }
}

pub async fn publications_handler() -> impl IntoResponse {
    let props = PublicationsProps::load().await;
    Html(PublicationsPage::render(&props).to_string())
}

#[component]
pub fn PublicationsPage(props: &PublicationsProps) -> Node {
    let total_publications = props.publications.len();
    let sync_status = if total_publications > 0 {
        format!("({total_publications} synced)")
    } else {
        String::from("(ready to sync)")
    };
    let notice_text = props
        .notice
        .as_deref()
        .unwrap_or("Add your publication profile ID and the page will populate automatically.");

    rsx! {
        <PageLayout title="Publications">
            <section class="py-4 md:py-8 space-y-8">
                <div class="space-y-4 max-w-4xl">
                    <div class="flex items-center gap-3 flex-wrap">
                        <h1 class="text-4xl md:text-5xl font-semibold text-white">"Publications"</h1>
                    </div>
                    <div class="section-rule"></div>
                    <p class="text-base md:text-lg text-zinc-300 leading-relaxed max-w-3xl">
                        "Selected writing and research notes across biodiagnostics, AI, and microfluidics. "
                        <span class="text-sm" style="color: var(--accent); opacity: 0.7;">{sync_status.as_str()}</span>
                    </p>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-[1fr_320px] gap-6 items-start">
                    <div class="space-y-4">
                        <div class="relative">
                            <i class="fas fa-search absolute left-4 top-1/2 -translate-y-1/2 text-zinc-500 text-sm pointer-events-none"></i>
                            <input
                                id="search-input"
                                type="text"
                                placeholder="Search publications..."
                                class="w-full bg-zinc-900/60 border border-zinc-800 rounded-xl pl-10 pr-4 py-3 text-sm text-zinc-200 placeholder-zinc-600 focus:outline-none focus:border-zinc-700 transition-colors"
                            />
                        </div>
                        {if props.publications.is_empty() {
                            rsx! {
                                <div class="card-item rounded-2xl p-6 space-y-3">
                                    <p class="text-sm text-zinc-300">"No publications are synced yet."</p>
                                    <p class="text-sm text-zinc-500 leading-relaxed">{notice_text}</p>
                                </div>
                            }
                        } else {
                            rsx! {
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                    {props.publications.iter().map(|publication| {
                                        let search_text = format!(
                                            "{} {} {}",
                                            publication.title,
                                            publication.venue.as_deref().unwrap_or(""),
                                            publication.work_type.as_deref().unwrap_or("")
                                        );
                                        <article data_searchtext={search_text.as_str()} class="card-item rounded-2xl p-5 space-y-4 soft-lift">
                                            <div class="flex items-start justify-between gap-3">
                                                <div class="space-y-2">
                                                    <p class="text-xs uppercase tracking-[0.1em] text-zinc-500">{publication.work_type.as_deref().unwrap_or("Publication")}</p>
                                                    <h2 class="text-xl font-semibold text-zinc-100 leading-snug">{&publication.title}</h2>
                                                </div>
                                                {if let Some(url) = &publication.url {
                                                    rsx! {
                                                        <a href={url} target="_blank" rel="noopener noreferrer" class="social-link shrink-0" aria_label="Open publication">
                                                            <i class="fas fa-arrow-up-right-from-square text-sm"></i>
                                                        </a>
                                                    }
                                                } else {
                                                    rsx! { <></> }
                                                }}
                                            </div>
                                            <div class="space-y-1 text-sm text-zinc-400">
                                                {if let Some(venue) = &publication.venue {
                                                    rsx! { <p>{venue}</p> }
                                                } else {
                                                    rsx! { <></> }
                                                }}
                                                {if let Some(published_on) = &publication.published_on {
                                                    rsx! { <p>{published_on}</p> }
                                                } else {
                                                    rsx! { <></> }
                                                }}
                                            </div>
                                        </article>
                                    })}
                                </div>
                            }
                        }}
                        <script>{r#"
                        (function(){
                          var s=document.getElementById('search-input');
                          if(!s)return;
                          s.addEventListener('input',function(){
                            var q=s.value.toLowerCase().trim();
                            document.querySelectorAll('[data-searchtext]').forEach(function(el){
                              el.style.display=!q||(el.getAttribute('data-searchtext')||'').toLowerCase().includes(q)?'':'none';
                            });
                          });
                        })();
                        "#}</script>
                    </div>

                    <aside class="space-y-4">
                        <div class="card-item rounded-2xl p-5 space-y-3">
                            <p class="text-xs uppercase tracking-[0.1em]" style="color: var(--accent);">"Focus areas"</p>
                            <div class="flex flex-wrap gap-2">
                                <span class="text-xs px-2.5 py-1 rounded-full border border-zinc-700 text-zinc-400">"Biodiagnostics"</span>
                                <span class="text-xs px-2.5 py-1 rounded-full border border-zinc-700 text-zinc-400">"AI"</span>
                                <span class="text-xs px-2.5 py-1 rounded-full border border-zinc-700 text-zinc-400">"Microfluidics"</span>
                            </div>
                        </div>
                    </aside>
                </div>
            </section>
        </PageLayout>
    }
}

fn normalize_orcid_id(orcid_id: &str) -> String {
    orcid_id
        .trim()
        .trim_start_matches("https://orcid.org/")
        .trim_start_matches("http://orcid.org/")
        .trim_matches('/')
        .to_string()
}

lazy_static::lazy_static! {
    static ref ORCID_CACHE: std::sync::Mutex<Option<(Vec<PublicationEntry>, i64)>> =
        std::sync::Mutex::new(None);
}

async fn fetch_publications(orcid_id: &str) -> Result<Vec<PublicationEntry>, String> {
    const TTL: i64 = 86400;
    let now = chrono::Utc::now().timestamp();

    if let Ok(guard) = ORCID_CACHE.lock() {
        if let Some((ref pubs, expiry)) = *guard {
            if now < expiry {
                return Ok(pubs.clone());
            }
        }
    }

    let client = reqwest::Client::new();
    let url = format!("https://pub.orcid.org/v3.0/{orcid_id}/works");
    let response = client
        .get(url)
        .header(reqwest::header::ACCEPT, "application/vnd.orcid+json")
        .send()
        .await
        .map_err(|error| error.to_string())?
        .error_for_status()
        .map_err(|error| error.to_string())?;

    let payload: Value = response.json().await.map_err(|error| error.to_string())?;
    let groups = payload
        .get("group")
        .or_else(|| payload.get("groups"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let publications: Vec<PublicationEntry> = groups
        .iter()
        .filter_map(|group| extract_publication_entry(group))
        .collect();

    if let Ok(mut guard) = ORCID_CACHE.lock() {
        *guard = Some((publications.clone(), now + TTL));
    }

    Ok(publications)
}

fn extract_publication_entry(group: &Value) -> Option<PublicationEntry> {
    let summary = first_summary(group)?;
    let title = string_at(
        summary,
        &[
            &["title", "title", "value"],
            &["work-title", "title", "value"],
            &["title", "value"],
            &["workTitle", "title", "value"],
        ],
    )?;
    let venue = string_at(
        summary,
        &[
            &["journal-title", "value"],
            &["journalTitle", "value"],
            &["venue", "value"],
        ],
    );
    let published_on = extract_publication_date(summary);
    let work_type = string_at(summary, &[&["type"], &["work-type"], &["workType"]])
        .map(|value| humanize_work_type(&value));
    let url = string_at(summary, &[&["url", "value"], &["url"], &["path"]]);

    Some(PublicationEntry {
        title,
        venue,
        published_on,
        work_type,
        url,
    })
}

fn first_summary(group: &Value) -> Option<&Value> {
    let summaries = group
        .get("work-summary")
        .or_else(|| group.get("workSummary"))
        .and_then(Value::as_array)?;

    summaries.first()
}

fn extract_publication_date(summary: &Value) -> Option<String> {
    let year = string_at(
        summary,
        &[
            &["publication-date", "year", "value"],
            &["publicationDate", "year", "value"],
        ],
    )?;
    let month = string_at(
        summary,
        &[
            &["publication-date", "month", "value"],
            &["publicationDate", "month", "value"],
        ],
    );
    let day = string_at(
        summary,
        &[
            &["publication-date", "day", "value"],
            &["publicationDate", "day", "value"],
        ],
    );

    Some(match (month, day) {
        (Some(month), Some(day)) => format!("{year}-{month:0>2}-{day:0>2}"),
        (Some(month), None) => format!("{year}-{month:0>2}"),
        _ => year,
    })
}

fn humanize_work_type(work_type: &str) -> String {
    work_type
        .split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn string_at(value: &Value, paths: &[&[&str]]) -> Option<String> {
    for path in paths {
        if let Some(found) = find_path(value, path).and_then(value_to_string) {
            return Some(found);
        }
    }
    None
}

fn find_path<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    Some(current)
}

fn value_to_string(value: &Value) -> Option<String> {
    match value {
        Value::String(text) => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        Value::Object(map) => {
            for key in ["value", "title", "name", "url", "path", "type"] {
                if let Some(text) = map.get(key).and_then(value_to_string) {
                    return Some(text);
                }
            }
            None
        }
        _ => None,
    }
}
