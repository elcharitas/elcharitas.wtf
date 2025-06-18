use crate::shared::SinglePostPublication;
use cookie::Cookie;
use ngyn::http::HeaderMap;
use reqwest::Client;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
struct AnalyticsEvent {
    event_type: String,
    time: u64,
    event_properties: AnalyticsEventProperties,
    device_id: String,
}

#[derive(Debug, Serialize)]
struct AnalyticsEventProperties {
    hostname: String,
    url: String,
    #[serde(rename = "eventType")]
    event_type: String,
    #[serde(rename = "publicationId")]
    publication_id: String,
    #[serde(rename = "dateAdded")]
    date_added: u64,
    referrer: Option<String>,
}

#[derive(Debug, Serialize)]
struct DashboardAnalyticsData {
    #[serde(rename = "publicationId")]
    publication_id: String,
    #[serde(rename = "postId")]
    post_id: String,
    timestamp: u64,
    url: String,
    referrer: Option<String>,
    title: String,
    charset: String,
    lang: String,
    #[serde(rename = "userAgent")]
    user_agent: String,
    #[serde(rename = "historyLength")]
    history_length: i32,
    #[serde(rename = "timezoneOffset")]
    timezone_offset: i32,
}

#[derive(Debug, Serialize)]
struct HashnodeAnalyticsRequest {
    events: Vec<AnalyticsEvent>,
}

#[derive(Debug, Serialize)]
struct HashnodeDashboardRequest {
    data: DashboardAnalyticsData,
}

pub async fn send_views_to_hashnode_internal_analytics(
    publication: Option<SinglePostPublication>,
    headers: HeaderMap,
    device_id: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let publication = match publication {
        Some(pub_data) => pub_data,
        None => return Ok(()),
    };

    let post = match &publication.post {
        Some(post_data) => post_data,
        None => return Ok(()),
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let referrer = headers
        .get("referer")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let event = AnalyticsEvent {
        event_type: "pageview".to_string(),
        time: now,
        event_properties: AnalyticsEventProperties {
            hostname: "elcharitas.wtf".to_string(),
            url: post.url.clone(),
            event_type: "pageview".to_string(),
            publication_id: publication.id.clone(),
            date_added: now,
            referrer,
        },
        device_id: device_id.to_string(),
    };

    let client = Client::new();
    let response = client
        .post("https://hn-ping2.hashnode.com/api/data-event")
        .header("Content-Type", "application/json")
        .header(
            reqwest::header::AUTHORIZATION,
            env::var("HASHNODE_TOKEN").unwrap_or_default(),
        )
        .json(&HashnodeAnalyticsRequest {
            events: vec![event],
        })
        .send()
        .await?;

    if !response.status().is_success() {
        eprintln!(
            "Failed to send to Hashnode Internal Analytics: {}",
            response.status()
        );
    }

    Ok(())
}

pub async fn send_views_to_hashnode_analytics_dashboard(
    publication: Option<SinglePostPublication>,
    headers: HeaderMap,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let publication = match publication {
        Some(pub_data) => pub_data,
        None => return Ok(()),
    };

    let post = match &publication.post {
        Some(post_data) => post_data,
        None => return Ok(()),
    };

    let referrer = headers
        .get("referer")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let accept_charset = headers
        .get("accept-charset")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("UTF-8")
        .to_string();

    let accept_language = headers
        .get("accept-language")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("en-US")
        .to_string();

    let data = DashboardAnalyticsData {
        publication_id: publication.id.clone(),
        post_id: post.id.clone(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        url: post.url.clone(),
        referrer,
        title: post.title.clone(),
        charset: accept_charset,
        lang: accept_language,
        user_agent,
        history_length: 1,
        timezone_offset: 0, // Default to 0 since we can't get client timezone on server
    };

    let client = Client::new();
    let response = client
        .post("https://hn-ping2.hashnode.com/api/view")
        .header("Content-Type", "application/json")
        .header(
            reqwest::header::AUTHORIZATION,
            env::var("HASHNODE_TOKEN").unwrap_or_default(),
        )
        .json(&HashnodeDashboardRequest { data })
        .send()
        .await?;

    if !response.status().is_success() {
        eprintln!(
            "Failed to send to Hashnode Analytics Dashboard: {}",
            response.status()
        );
    }

    Ok(())
}

pub fn get_or_create_device_id(request_cookies: &str) -> (String, Option<Cookie<'static>>) {
    const COOKIE_NAME: &str = "__amplitudeDeviceID";

    if let Some(existing_value) = parse_cookie_value(request_cookies, COOKIE_NAME) {
        return (existing_value, None);
    }

    let device_id = uuid::Uuid::new_v4().to_string().replace("-", "");

    // Create cookie that expires in 2 years (730 days)
    let mut cookie = Cookie::new(COOKIE_NAME, device_id.clone());
    cookie.set_max_age(cookie::time::Duration::days(730));
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_secure(true); // Set to true for HTTPS
    cookie.set_same_site(cookie::SameSite::Lax);

    (device_id, Some(cookie))
}

fn parse_cookie_value(cookie_header: &str, name: &str) -> Option<String> {
    cookie_header
        .split(';')
        .map(|s| s.trim())
        .find_map(|cookie_pair| {
            let mut parts = cookie_pair.splitn(2, '=');
            let cookie_name = parts.next()?.trim();
            let cookie_value = parts.next()?.trim();

            if cookie_name == name {
                Some(cookie_value.to_string())
            } else {
                None
            }
        })
}
