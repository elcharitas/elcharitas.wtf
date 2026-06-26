use axum::extract::Query;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

// ------ env var abstraction ------

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
use worker::kv::KvStore;

#[cfg(target_arch = "wasm32")]
thread_local! {
    static ENV_VARS: RefCell<std::collections::HashMap<String, String>> = RefCell::new(std::collections::HashMap::new());
    static NEWSLETTER_KV: RefCell<Option<KvStore>> = RefCell::new(None);
}

#[cfg(target_arch = "wasm32")]
pub fn init_env(env: &worker::Env) {
    ENV_VARS.with(|vars| {
        let mut map = vars.borrow_mut();
        for key in ["GITHUB_TOKEN", "ORCID_ID", "ENVIRONMENT", "RESEND_API_KEY"] {
            let value = env.secret(key)
                .map(|s| s.to_string())
                .or_else(|_| env.var(key).map(|v| v.to_string()))
                .unwrap_or_default();
            map.insert(key.to_string(), value);
        }
    });
}

#[cfg(target_arch = "wasm32")]
pub fn init_kv(env: &worker::Env) {
    NEWSLETTER_KV.with(|kv| {
        match env.kv("NEWSLETTER_SUBSCRIBERS") {
            Ok(store) => {
                worker::console_log!("newsletter: KV binding initialised");
                *kv.borrow_mut() = Some(store);
            }
            Err(e) => {
                worker::console_log!("newsletter: KV binding failed: {:?}", e);
                *kv.borrow_mut() = None;
            }
        }
    });
}

#[cfg(target_arch = "wasm32")]
pub fn get_newsletter_kv() -> Option<KvStore> {
    NEWSLETTER_KV.with(|kv| kv.borrow().clone())
}

pub fn get_env(key: &str) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        ENV_VARS.with(|vars| {
            vars.borrow().get(key).cloned().unwrap_or_default()
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::var(key).unwrap_or_default()
    }
}

// ------ end env var abstraction ------

#[derive(Deserialize)]
pub struct PageParams {
    pub slug: String,
}

#[derive(Default, Deserialize)]
pub struct PageQuery {
    pub has_next_page: bool,
    pub cursor: String,
}

impl PageQuery {
    pub fn from_query(query: Query<serde_json::Value>) -> Self {
        let datastar: String = query
            .0
            .get("datastar")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        serde_json::from_str(&datastar).unwrap_or_default()
    }
}

pub struct NavigationInfo {
    pub name: &'static str,
    pub href: &'static str,
}

lazy_static! {
    pub static ref NAVIGATION: Vec<NavigationInfo> = vec![
        NavigationInfo { name: "Projects", href: "/projects" },
        NavigationInfo { name: "Essays", href: "/essays" },
        NavigationInfo { name: "Publications", href: "/publications" },
        NavigationInfo { name: "Resume", href: "/resume" },
        NavigationInfo { name: "Timeline", href: "/adventures" },
    ];
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Content {
    pub markdown: String,
    pub html: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PostCoverImage {
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageInfo {
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenGraphMetaData {
    pub image: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SEO {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCommentConnection {
    pub edges: Option<Vec<()>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub slug: String,
    pub url: String,
    pub brief: String,
    pub title: String,
    pub subtitle: Option<String>,
    #[serde(rename = "publishedAt")]
    pub published_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "readTimeInMinutes")]
    pub read_time_in_minutes: i32,
    #[serde(rename = "reactionCount")]
    pub reaction_count: Option<i32>,
    #[serde(rename = "responseCount")]
    pub response_count: Option<i32>,
    pub views: Option<i32>,
    pub seo: Option<SEO>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<PostCoverImage>,
    pub author: Option<serde_json::Value>,
    pub content: Option<Content>,
    #[serde(rename = "ogMetaData")]
    pub og_meta_data: Option<OpenGraphMetaData>,
    pub tags: Vec<Tag>,
    pub comments: Option<PostCommentConnection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub url: String,
    pub image: Option<String>,
    pub tags: Vec<String>,
    pub stargazers_count: f32,
    pub language: Option<String>,
    pub updated_at: String,
    pub homepage: String,
    pub watching: f32,
}

pub mod xml_elements {
    pub mod dom {
        pub mod elements {
            use momenta::derive_elements;
            extern crate alloc;

            derive_elements! {
                rss {}
                channel {}
                loc {}
                title {}
                description {}
                link {}
                lastmod {}
                changefreq {}
                priority {}
                image {}
                lastBuildDate {}
                item {}
                guid {}
                pubDate {}
                sitemapindex {
                    xmlns: String,
                }
                sitemap {}
            }
        }
    }
}
