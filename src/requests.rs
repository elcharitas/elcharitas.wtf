use reqwest;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};
use thiserror::Error;

use crate::shared::Project;

// GraphQL fragments (these would typically be loaded from .graphql files)
const PAGE_INFO_FRAGMENT: &str = include_str!("../graphql/fragments/PageInfo.graphql");
const POST_FRAGMENT: &str = include_str!("../graphql/fragments/Post.graphql");
const PUBLICATION_FRAGMENT: &str = include_str!("../graphql/fragments/Publication.graphql");

/// Represents a GraphQL error location
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorLocation {
    pub line: u32,
    pub column: u32,
}

/// Represents GraphQL error extensions
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorExtensions {
    pub code: String,
}

/// Represents a GraphQL error
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GraphQLError {
    pub message: String,
    pub locations: Option<Vec<ErrorLocation>>,
    pub path: Option<Vec<serde_json::Value>>,
    pub extensions: Option<ErrorExtensions>,
}

/// Represents a GraphQL API response
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

/// Configuration options for GraphQL requests
#[derive(Debug, Clone)]
pub struct QueryOptions {
    pub base_url: String,
    pub headers: HashMap<String, String>,
    pub timeout_seconds: Option<u64>,
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            base_url: "https://gql.hashnode.com".to_string(),
            headers: HashMap::new(),
            timeout_seconds: Some(30), // 30 second default timeout
        }
    }
}

/// Error types for GraphQL operations
#[derive(Error, Debug)]
pub enum GraphQLClientError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization failed: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("GraphQL errors: {0:?}")]
    GraphQLErrors(Vec<GraphQLError>),

    #[error("No data returned from GraphQL API")]
    NoData,

    #[error("Client building failed: {0}")]
    ClientBuildError(String),
}

/// Represents a GraphQL query with operation name
#[derive(Debug, Clone)]
pub struct GraphQLQuery {
    pub operation_name: String,
    pub query: String,
}

impl GraphQLQuery {
    pub fn new(operation_name: impl Into<String>, query: impl Into<String>) -> Self {
        Self {
            operation_name: operation_name.into(),
            query: query.into(),
        }
    }
}

/// Asynchronous GraphQL client for executing queries
pub struct GraphQLClient {
    client: reqwest::Client,
    options: QueryOptions,
}

impl GraphQLClient {
    /// Create a new GraphQL client with default options
    pub fn new() -> Result<Self, GraphQLClientError> {
        let options = QueryOptions::default();
        Self::with_options(options)
    }

    /// Create a new GraphQL client with custom options
    pub fn with_options(options: QueryOptions) -> Result<Self, GraphQLClientError> {
        let mut client_builder = reqwest::Client::builder();

        // Set timeout if specified
        if let Some(timeout) = options.timeout_seconds {
            client_builder = client_builder.timeout(std::time::Duration::from_secs(timeout));
        }

        let client = client_builder
            .build()
            .map_err(|e| GraphQLClientError::ClientBuildError(e.to_string()))?;

        Ok(Self { client, options })
    }

    /// Build a GraphQL query by prepending necessary fragments
    fn build_query(&self, query: &str) -> String {
        let mut final_query = Vec::new();

        // Add fragments if they're referenced in the query
        if query.contains("PageInfo") {
            final_query.push(PAGE_INFO_FRAGMENT);
        }
        if query.contains("Post") {
            final_query.push(POST_FRAGMENT);
        }
        if query.contains("Publication") {
            final_query.push(PUBLICATION_FRAGMENT);
        }

        // Add the main query
        final_query.push(query);

        final_query.join("\n")
    }

    /// Execute a GraphQL query asynchronously
    pub async fn execute_query<T>(
        &self,
        query: GraphQLQuery,
        variables: Option<serde_json::Value>,
    ) -> Result<T, GraphQLClientError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let built_query = self.build_query(&query.query);

        // Prepare request body
        let request_body = serde_json::json!({
            "query": built_query,
            "variables": variables.unwrap_or(serde_json::Value::Null),
            "operationName": query.operation_name,
        });

        // Build headers
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            env::var("HASHNODE_TOKEN")
                .unwrap_or_default()
                .parse()
                .unwrap(),
        );

        // Add custom headers
        for (key, value) in &self.options.headers {
            if let (Ok(header_name), Ok(header_value)) = (
                reqwest::header::HeaderName::from_bytes(key.as_bytes()),
                reqwest::header::HeaderValue::from_str(value),
            ) {
                headers.insert(header_name, header_value);
            }
        }

        // Execute request asynchronously
        let response = self
            .client
            .post(&self.options.base_url)
            .headers(headers)
            .json(&request_body)
            .send()
            .await?;

        // Parse response
        let api_response: ApiResponse<T> = response.json().await?;

        // Handle GraphQL errors
        if let Some(errors) = api_response.errors {
            return Err(GraphQLClientError::GraphQLErrors(errors));
        }

        // Return data or error if no data
        api_response.data.ok_or(GraphQLClientError::NoData)
    }
}

impl Default for GraphQLClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default GraphQL client")
    }
}

// Convenience macros for creating queries
#[macro_export]
macro_rules! graphql_query {
    ($operation_name:ident, $query:expr) => {
        GraphQLQuery::new(stringify!($operation_name), $query)
    };
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

#[derive(Debug)]
pub enum GitHubError {
    RequestError(reqwest::Error),
    EnvError(env::VarError),
    JsonError(reqwest::Error),
}

impl std::fmt::Display for GitHubError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GitHubError::RequestError(e) => write!(f, "Request error: {}", e),
            GitHubError::EnvError(e) => write!(f, "Environment variable error: {}", e),
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

impl From<env::VarError> for GitHubError {
    fn from(error: env::VarError) -> Self {
        GitHubError::EnvError(error)
    }
}

pub async fn get_all_projects(page: u32) -> Result<Vec<Project>, GitHubError> {
    let github_token = env::var("GITHUB_TOKEN").unwrap_or("".to_string());
    let client = reqwest::Client::builder().build()?;

    let url = format!(
        "https://api.github.com/user/repos?sort=updated&visibility=public&affiliation=owner,collaborator&per_page=21&direction=desc&page={}",
        page
    );
    let response = client
        .get(&url)
        .header("Authorization", format!("token {}", github_token))
        .header("User-Agent", "Rust-GitHub-Client") // GitHub requires a User-Agent
        .send()
        .await?;

    if !response.status().is_success() {
        return Ok(Vec::new()); // Return empty vector on failure, similar to original
    }

    let repos: Vec<GitHubRepo> = response.json().await.map_err(GitHubError::JsonError)?;

    let mut filtered_repos: Vec<GitHubRepo> = repos
        .into_iter()
        .filter(|repo| !repo.fork && repo.stargazers_count > 0.0)
        .collect();

    // Sort by star count (descending) and by last updated
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_all_projects() {
        // This test requires GITHUB_TOKEN environment variable
        if env::var("GITHUB_TOKEN").is_ok() {
            let result = get_all_projects(1).await;
            match result {
                Ok(projects) => {
                    println!("Found {} projects", projects.len());
                    for project in projects.iter().take(3) {
                        println!(
                            "Project: {} - Stars: {}",
                            project.name, project.stargazers_count
                        );
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        } else {
            println!("Skipping test - GITHUB_TOKEN not set");
        }
    }

    #[test]
    fn test_build_query_with_fragments() {
        let client = GraphQLClient::new().unwrap();
        let query = r#"
            query GetPosts {
                posts {
                    ...Post
                    pageInfo {
                        ...PageInfo
                    }
                }
            }
        "#;

        let built_query = client.build_query(query);

        assert!(built_query.contains("fragment PageInfo"));
        assert!(built_query.contains("fragment Post"));
        assert!(!built_query.contains("fragment Publication"));
    }

    #[test]
    fn test_query_creation() {
        let query = GraphQLQuery::new("GetUser", "query GetUser { user { id name } }");
        assert_eq!(query.operation_name, "GetUser");
        assert_eq!(query.query, "query GetUser { user { id name } }");
    }

    #[test]
    fn test_client_creation_with_options() {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token".to_string());

        let options = QueryOptions {
            base_url: "https://api.example.com/graphql".to_string(),
            headers,
            timeout_seconds: Some(60),
        };

        let client = GraphQLClient::with_options(options).unwrap();
        assert_eq!(client.options.base_url, "https://api.example.com/graphql");
        assert!(client.options.headers.contains_key("Authorization"));
        assert_eq!(client.options.timeout_seconds, Some(60));
    }

    #[test]
    fn test_macro() {
        let query = graphql_query!(TestOp, "query TestOp { test }");
        assert_eq!(query.operation_name, "TestOp");
        assert_eq!(query.query, "query TestOp { test }");
    }
}
