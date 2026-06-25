use reqwest;
use serde::Deserialize;

use crate::shared::{get_env, Content, Post, Project};

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

#[derive(Debug, Deserialize)]
struct GitHubFile {
    name: String,
    download_url: Option<String>,
}

pub fn parse_post_from_markdown(slug: &str, markdown: &str) -> Post {
    let mut title = slug.to_string();
    let mut found_title = false;
    let mut content_after_title = String::new();

    for line in markdown.lines() {
        if !found_title {
            if let Some(t) = line.strip_prefix("# ") {
                title = t.trim().to_string();
                found_title = true;
            }
        } else {
            content_after_title.push_str(line);
            content_after_title.push('\n');
        }
    }

    let brief = content_after_title
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with("```"))
        .next()
        .unwrap_or("")
        .chars()
        .take(200)
        .collect::<String>();

    let word_count = markdown.split_whitespace().count();
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
            markdown: markdown.to_string(),
            html: String::new(),
        }),
        og_meta_data: None,
        tags: Vec::new(),
        comments: None,
    }
}

fn github_headers(token: &str) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    if let Ok(v) = format!("token {}", token).parse() {
        headers.insert(reqwest::header::AUTHORIZATION, v);
    }
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("elcharitas-wtf"),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/vnd.github+json"),
    );
    headers
}

pub async fn fetch_all_posts() -> Vec<Post> {
    let token = get_env("GITHUB_TOKEN");
    let client = match reqwest::Client::builder().build() {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let resp = match client
        .get("https://api.github.com/repos/elcharitas/hashnode-blog-backup/contents")
        .headers(github_headers(&token))
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => return Vec::new(),
    };

    let text = match resp.text().await {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };

    let files: Vec<GitHubFile> = match serde_json::from_str(&text) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let md_files: Vec<GitHubFile> = files
        .into_iter()
        .filter(|f| f.name.ends_with(".md"))
        .collect();

    let mut posts = Vec::new();

    for file in md_files {
        let slug = file.name.trim_end_matches(".md").to_string();
        let download_url = match file.download_url {
            Some(u) => u,
            None => continue,
        };
        let resp = match client
            .get(&download_url)
            .headers(github_headers(&token))
            .send()
            .await
        {
            Ok(r) => r,
            Err(_) => continue,
        };
        let markdown = match resp.text().await {
            Ok(c) => c,
            Err(_) => continue,
        };
        let mut post = parse_post_from_markdown(&slug, &markdown);
        post.content = None;
        posts.push(post);
    }

    posts.reverse();
    posts
}

pub async fn fetch_post_by_slug_from_github(slug: &str) -> Option<Post> {
    let token = get_env("GITHUB_TOKEN");
    let client = reqwest::Client::builder().build().ok()?;

    let url = format!(
        "https://raw.githubusercontent.com/elcharitas/hashnode-blog-backup/main/{}.md",
        slug
    );
    let resp = client
        .get(&url)
        .headers(github_headers(&token))
        .send()
        .await
        .ok()?;

    if !resp.status().is_success() {
        return None;
    }

    let markdown = resp.text().await.ok()?;
    Some(parse_post_from_markdown(slug, &markdown))
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
