use reqwest;
use serde::Deserialize;

use crate::shared::{Content, Post, Project, Tag, get_env};

#[derive(Debug, Deserialize)]
struct PostMeta {
    slug: String,
    title: String,
    brief: String,
    read_time_in_minutes: i32,
    category: Option<String>,
    published_at: Option<String>,
}

fn post_meta_to_post(m: PostMeta) -> Post {
    let tags = m
        .category
        .map(|c| {
            vec![Tag {
                id: c.clone(),
                name: c.clone(),
                slug: c,
            }]
        })
        .unwrap_or_default();
    Post {
        id: m.slug.clone(),
        url: format!("https://elcharitas.wtf/essays/{}", m.slug),
        slug: m.slug,
        title: m.title,
        brief: m.brief,
        subtitle: None,
        published_at: m.published_at,
        updated_at: None,
        read_time_in_minutes: m.read_time_in_minutes,
        reaction_count: None,
        response_count: None,
        views: None,
        seo: None,
        cover_image: None,
        author: None,
        content: None,
        og_meta_data: None,
        tags,
        comments: None,
    }
}

#[cfg(target_arch = "wasm32")]
async fn http_get_text(url: &str) -> Option<String> {
    use worker::{Fetch, Method, Request, RequestInit};
    let req = Request::new_with_init(url, RequestInit::new().with_method(Method::Get)).ok()?;
    let mut resp = Fetch::Request(req).send().await.ok()?;
    if resp.status_code() >= 400 {
        return None;
    }
    resp.text().await.ok()
}

#[cfg(not(target_arch = "wasm32"))]
async fn http_get_text(url: &str) -> Option<String> {
    let client = reqwest::Client::builder().build().ok()?;
    let resp = client
        .get(url)
        .header("User-Agent", "elcharitas-wtf")
        .send()
        .await
        .ok()?;
    if !resp.status().is_success() {
        return None;
    }
    resp.text().await.ok()
}

pub async fn fetch_all_posts() -> Vec<Post> {
    let url = "https://raw.githubusercontent.com/elcharitas/elcharitas.wtf/main/blog/posts.json";
    let json = http_get_text(url).await.unwrap_or_default();
    serde_json::from_str::<Vec<PostMeta>>(&json)
        .unwrap_or_default()
        .into_iter()
        .map(post_meta_to_post)
        .collect()
}

fn strip_frontmatter(markdown: &str) -> &str {
    if markdown.starts_with("---") {
        if let Some(end) = markdown[3..].find("\n---") {
            return markdown[3 + end + 4..].trim_start();
        }
    }
    markdown
}

fn parse_frontmatter_field<'a>(markdown: &'a str, key: &str) -> Option<&'a str> {
    if !markdown.starts_with("---") {
        return None;
    }
    let end = markdown[3..].find("\n---")?;
    let fm = &markdown[3..3 + end];
    for line in fm.lines() {
        if let Some(rest) = line.strip_prefix(&format!("{key}:")) {
            return Some(rest.trim().trim_matches('"'));
        }
    }
    None
}

fn parse_title_from_frontmatter(markdown: &str) -> Option<String> {
    parse_frontmatter_field(markdown, "title").map(|s| s.to_string())
}

pub fn parse_post_from_markdown(slug: &str, raw: &str) -> Post {
    let title = parse_title_from_frontmatter(raw).unwrap_or_else(|| {
        let body = strip_frontmatter(raw);
        body.lines()
            .find_map(|l| l.trim().strip_prefix("# ").map(|t| t.trim().to_string()))
            .unwrap_or_else(|| slug.to_string())
    });

    let body = strip_frontmatter(raw);
    let brief = body
        .lines()
        .map(|l| l.trim())
        .filter(|l| {
            !l.is_empty() && !l.starts_with('#') && !l.starts_with("```") && !l.starts_with('!')
        })
        .next()
        .unwrap_or("")
        .chars()
        .take(200)
        .collect::<String>();

    let word_count = body.split_whitespace().count();
    let read_time = ((word_count as f32) / 200.0).ceil() as i32;

    Post {
        id: slug.to_string(),
        slug: slug.to_string(),
        url: format!("https://elcharitas.wtf/essays/{}", slug),
        title,
        brief,
        subtitle: None,
        published_at: None,
        updated_at: None,
        read_time_in_minutes: read_time.max(1),
        reaction_count: None,
        response_count: None,
        views: None,
        seo: None,
        cover_image: None,
        author: None,
        content: Some(Content {
            markdown: body.to_string(),
            html: String::new(),
        }),
        og_meta_data: None,
        tags: Vec::new(),
        comments: None,
    }
}

pub async fn fetch_post_by_slug_from_github(slug: &str) -> Option<Post> {
    let url = format!(
        "https://raw.githubusercontent.com/elcharitas/elcharitas.wtf/main/blog/{}.md",
        slug
    );
    let raw = http_get_text(&url).await?;
    Some(parse_post_from_markdown(slug, &raw))
}

// ---- GitHub projects API ----

#[derive(Debug)]
pub enum GitHubError {
    RequestError(reqwest::Error),
    JsonError(reqwest::Error),
}

impl std::fmt::Display for GitHubError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GitHubError::RequestError(e) => write!(f, "Request error: {}", e),
            GitHubError::JsonError(e) => write!(f, "JSON parsing error: {}", e),
        }
    }
}

impl std::error::Error for GitHubError {}

impl From<reqwest::Error> for GitHubError {
    fn from(error: reqwest::Error) -> Self {
        GitHubError::RequestError(error)
    }
}

#[derive(Debug, Deserialize)]
struct GitHubRepo {
    name: String,
    description: Option<String>,
    stargazers_count: f32,
    fork: bool,
    pushed_at: String,
    homepage: Option<String>,
    html_url: String,
    default_branch: String,
    owner: GitHubOwner,
    topics: Vec<String>,
    watchers_count: f32,
}

#[derive(Debug, Deserialize)]
struct GitHubOwner {
    login: String,
}

pub async fn get_all_projects(page: u32) -> Result<Vec<Project>, GitHubError> {
    let github_token = get_env("GITHUB_TOKEN");
    let client = reqwest::Client::builder().build()?;

    let url = format!(
        "https://api.github.com/user/repos?sort=updated&visibility=public&affiliation=owner,collaborator&per_page=25&direction=desc&page={}",
        page
    );
    let response = client
        .get(&url)
        .header("Authorization", format!("token {}", github_token))
        .header("User-Agent", "Rust-GitHub-Client")
        .send()
        .await?;

    if !response.status().is_success() {
        return Ok(Vec::new());
    }

    let repos: Vec<GitHubRepo> = response.json().await.map_err(GitHubError::JsonError)?;

    let mut filtered_repos: Vec<GitHubRepo> = repos
        .into_iter()
        .filter(|repo| !repo.fork && repo.stargazers_count > 0.0)
        .collect();

    filtered_repos.sort_by(|a, b| {
        (b.stargazers_count as u32)
            .cmp(&(a.stargazers_count as u32))
            .then_with(|| b.pushed_at.cmp(&a.pushed_at))
    });

    let projects: Vec<Project> = filtered_repos
        .into_iter()
        .map(|repo| {
            let cover_image_url = format!(
                "https://raw.githubusercontent.com/{}/{}/{}/static/logo-light.svg",
                repo.owner.login, repo.name, repo.default_branch
            );
            Project {
                url: repo.html_url,
                name: repo.name,
                description: repo.description.unwrap_or_default(),
                stargazers_count: repo.stargazers_count,
                language: None,
                updated_at: repo.pushed_at,
                homepage: repo.homepage.unwrap_or_default(),
                image: Some(cover_image_url),
                tags: repo.topics,
                watching: 100.0
                    * (if repo.stargazers_count > repo.watchers_count {
                        repo.watchers_count / (repo.stargazers_count + 0.005)
                    } else {
                        repo.stargazers_count / (repo.watchers_count + 0.005)
                    }),
            }
        })
        .collect();

    Ok(projects)
}
