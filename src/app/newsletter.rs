use crate::components::PageLayout;
use axum::{
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{Html, IntoResponse, Response},
};
use hmac::{Hmac, Mac};
use momenta::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

const CSRF_COOKIE_NAME: &str = "__Host-newsletter_csrf";
const CSRF_TOKEN_TTL_SECS: i64 = 15 * 60;
type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NewsletterSubscription {
    pub email: String,
    pub csrf_token: String,
    pub csrf_expires: String,
}

impl NewsletterSubscription {
    fn from_body(body: &str) -> Self {
        let mut subscription = Self::default();
        for (key, value) in url::form_urlencoded::parse(body.as_bytes()) {
            match key.as_ref() {
                "email" => subscription.email = value.to_string(),
                "csrf_token" => subscription.csrf_token = value.to_string(),
                "csrf_expires" => subscription.csrf_expires = value.to_string(),
                _ => {}
            }
        }
        subscription
    }
}

fn csrf_token(secret: &str, nonce: &str, expires: i64) -> Option<String> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).ok()?;
    mac.update(format!("newsletter:{nonce}:{expires}").as_bytes());
    Some(
        mac.finalize()
            .into_bytes()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect(),
    )
}

fn csrf_cookie(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|cookie| {
                let (name, value) = cookie.trim().split_once('=')?;
                (name == CSRF_COOKIE_NAME).then_some(value)
            })
        })
}

fn decode_hex(input: &str) -> Option<Vec<u8>> {
    if input.len() != 64 {
        return None;
    }

    input
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = (pair[0] as char).to_digit(16)? as u8;
            let low = (pair[1] as char).to_digit(16)? as u8;
            Some((high << 4) | low)
        })
        .collect()
}

fn valid_csrf_token_with_secret(
    secret: &str,
    headers: &HeaderMap,
    subscription: &NewsletterSubscription,
) -> bool {
    let Some(nonce) = csrf_cookie(headers) else {
        return false;
    };
    let Ok(expires) = subscription.csrf_expires.parse::<i64>() else {
        return false;
    };
    let now = chrono::Utc::now().timestamp();
    if secret.is_empty() || expires < now || expires > now + CSRF_TOKEN_TTL_SECS {
        return false;
    }
    let Some(signature) = decode_hex(&subscription.csrf_token) else {
        return false;
    };
    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(format!("newsletter:{nonce}:{expires}").as_bytes());
    mac.verify_slice(&signature).is_ok()
}

fn valid_csrf_token(headers: &HeaderMap, subscription: &NewsletterSubscription) -> bool {
    valid_csrf_token_with_secret(
        &crate::shared::get_env("CSRF_SECRET"),
        headers,
        subscription,
    )
}

fn newsletter_form_response() -> Response {
    let secret = crate::shared::get_env("CSRF_SECRET");
    if secret.is_empty() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "Newsletter protection is temporarily unavailable. Please try again shortly.",
        )
            .into_response();
    }

    let nonce = uuid::Uuid::new_v4().to_string();
    let expires = chrono::Utc::now().timestamp() + CSRF_TOKEN_TTL_SECS;
    let Some(token) = csrf_token(&secret, &nonce, expires) else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "Newsletter protection is temporarily unavailable. Please try again shortly.",
        )
            .into_response();
    };
    let props = NewsletterSubscription {
        csrf_token: token,
        csrf_expires: expires.to_string(),
        ..Default::default()
    };
    let mut response = Html(NewsletterPage::render(&props).to_string()).into_response();
    let cookie = format!(
        "{CSRF_COOKIE_NAME}={nonce}; Path=/; Max-Age={CSRF_TOKEN_TTL_SECS}; HttpOnly; SameSite=Strict; Secure"
    );
    if let Ok(cookie) = HeaderValue::from_str(&cookie) {
        response.headers_mut().append(header::SET_COOKIE, cookie);
    } else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "Newsletter protection is temporarily unavailable. Please try again shortly.",
        )
            .into_response();
    }
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

pub async fn newsletter_send_handler(
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let token = crate::shared::get_env("NEWSLETTER_SEND_TOKEN");
    let auth = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !token.is_empty() && auth != format!("Bearer {token}") {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    }
    #[cfg(target_arch = "wasm32")]
    send_newsletter().await;
    axum::http::StatusCode::OK.into_response()
}

pub async fn newsletter_get_handler() -> Response {
    newsletter_form_response()
}

pub async fn newsletter_post_handler(headers: HeaderMap, body: String) -> Response {
    let props = NewsletterSubscription::from_body(&body);
    if !valid_csrf_token(&headers, &props) {
        return (
            StatusCode::FORBIDDEN,
            "Invalid or expired newsletter form. Refresh the page and try again.",
        )
            .into_response();
    }

    #[cfg(target_arch = "wasm32")]
    if !props.email.is_empty() {
        worker::console_log!("newsletter: handling subscription for {}", props.email);
        let api_key = crate::shared::get_env("RESEND_API_KEY");
        match crate::shared::get_newsletter_kv() {
            None => worker::console_log!("newsletter: KV not available, skipping insert"),
            Some(kv) => match kv.put(&format!("subscriber:{}", props.email), "1") {
                Err(e) => worker::console_log!("newsletter: kv.put() error: {:?}", e),
                Ok(builder) => match builder.execute().await {
                    Ok(_) => worker::console_log!("newsletter: {} saved to KV", props.email),
                    Err(e) => worker::console_log!("newsletter: kv.execute() error: {:?}", e),
                },
            },
        }
        if api_key.is_empty() {
            worker::console_log!("newsletter: RESEND_API_KEY not set, skipping welcome email");
        } else {
            send_welcome_email(&props.email, &api_key).await;
        }
    }

    Html(NewsletterPage::render(&props).to_string()).into_response()
}

#[cfg(target_arch = "wasm32")]
fn from_data() -> String {
    let from_email = crate::shared::get_env("RESEND_FROM_EMAIL");
    let from_name = crate::shared::get_env("RESEND_FROM_NAME");
    let from = if !from_name.is_empty() {
        format!("{} <{}>", from_name, from_email)
    } else {
        "Jonathan Irhodia <newsletter@elcharitas.wtf>".to_string()
    };
    from
}

#[cfg(target_arch = "wasm32")]
fn render_more_posts_section(posts: &[&crate::shared::Post]) -> String {
    if posts.is_empty() {
        return String::new();
    }

    let items: String = posts
        .iter()
        .map(|p| {
            format!(
                r##"<mj-text padding-bottom="4px">
                  <a href="{url}" style="color:#09090b;text-decoration:none;font-weight:600;font-size:15px;">{title}</a>
                </mj-text>
                <mj-text color="#71717a" font-size="13px" padding-bottom="16px">
                  {brief}
                </mj-text>"##,
                url = p.url,
                title = p.title,
                brief = p.brief,
            )
        })
        .collect();

    format!(
        r##"<mj-section background-color="#ffffff" padding="0 0 32px" border-radius="0 0 8px 8px">
      <mj-column padding="0 32px">
        <mj-divider border-color="#e4e4e7" border-width="1px" padding-bottom="24px" />
        <mj-text font-size="11px" color="#a1a1aa" font-weight="600" letter-spacing="0.1em" text-transform="uppercase" padding-bottom="16px">
          Also This Week
        </mj-text>
        {items}
      </mj-column>
    </mj-section>"##
    )
}

#[cfg(target_arch = "wasm32")]
fn render_email(
    title: &str,
    brief: &str,
    content: &str,
    url: &str,
    intro: &str,
    more_posts: &[&crate::shared::Post],
) -> String {
    let more_posts_section = render_more_posts_section(more_posts);
    let card_radius = if more_posts.is_empty() { "0 0 8px 8px" } else { "0" };

    let template = include_str!("../emails/newsletter.mrml")
        .replace("{{subject}}", &format!("Weekly: {title}"))
        .replace("{{intro}}", intro)
        .replace("{{title}}", title)
        .replace("{{brief}}", brief)
        .replace("{{content}}", content)
        .replace("{{url}}", url)
        .replace("{{card_radius}}", card_radius)
        .replace("{{more_posts_section}}", &more_posts_section);

    let opts = mrml::prelude::render::RenderOptions::default();
    mrml::parse(&template)
        .ok()
        .and_then(|root| root.element.render(&opts).ok())
        .unwrap_or_else(|| {
            format!(
                "<h2>{title}</h2><p>{brief}</p><p><a href=\"{url}\">Read →</a></p>\
                 <p><a href=\"https://elcharitas.wtf/newsletter\">Unsubscribe</a></p>"
            )
        })
}

#[cfg(target_arch = "wasm32")]
fn render_welcome_email() -> String {
    let template = include_str!("../emails/welcome.mrml");
    let opts = mrml::prelude::render::RenderOptions::default();
    mrml::parse(template)
        .ok()
        .and_then(|root| root.element.render(&opts).ok())
        .unwrap_or_else(|| {
            "<h2>Welcome aboard.</h2>\
             <p>Thanks for subscribing to my weekly field notes from the build process.</p>\
             <p><a href=\"https://elcharitas.wtf/essays\">Read latest essays →</a></p>\
             <p><a href=\"https://elcharitas.wtf/newsletter\">Unsubscribe</a></p>"
                .to_string()
        })
}

#[cfg(target_arch = "wasm32")]
async fn send_welcome_email(email: &str, api_key: &str) {
    let html = render_welcome_email();
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "from": from_data(),
        "to": [email],
        "subject": "Welcome to the newsletter",
        "html": html,
    });
    match client
        .post("https://api.resend.com/emails")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await
    {
        Ok(resp) => worker::console_log!(
            "newsletter: welcome email sent to {} (status {})",
            email,
            resp.status().as_u16()
        ),
        Err(e) => worker::console_log!(
            "newsletter: failed to send welcome email to {}: {:?}",
            email,
            e
        ),
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn send_newsletter() {
    let Some(kv) = crate::shared::get_newsletter_kv() else {
        return;
    };
    let api_key = crate::shared::get_env("RESEND_API_KEY");
    if api_key.is_empty() {
        return;
    }

    let list = match kv.list().prefix("subscriber:".to_string()).execute().await {
        Ok(l) => l,
        Err(_) => return,
    };

    let emails: Vec<String> = list
        .keys
        .iter()
        .map(|k| k.name.trim_start_matches("subscriber:").to_string())
        .collect();

    if emails.is_empty() {
        return;
    }

    let posts = crate::requests::fetch_all_posts().await;
    let one_week_ago = chrono::Utc::now() - chrono::Duration::days(7);
    let recent: Vec<_> = posts
        .iter()
        .filter(|p| {
            p.published_at
                .as_deref()
                .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc() >= one_week_ago)
                .unwrap_or(false)
        })
        .collect();

    if recent.is_empty() {
        return;
    }

    let count = recent.len();
    let latest = recent[0];
    let more_posts: Vec<_> = recent.iter().skip(1).take(3).copied().collect();

    let content_html = crate::requests::fetch_post_by_slug_from_github(&latest.slug)
        .await
        .and_then(|p| p.content)
        .map(|c| {
            let opts = comrak::Options {
                render: comrak::RenderOptions {
                    unsafe_: true,
                    ..Default::default()
                },
                extension: comrak::ExtensionOptions {
                    table: true,
                    strikethrough: true,
                    autolink: true,
                    tasklist: true,
                    ..Default::default()
                },
                ..Default::default()
            };
            comrak::markdown_to_html(&c.markdown, &opts)
        })
        .unwrap_or_default();

    let intro = if count == 1 {
        "This past week I shipped one new post. Here's what it's about:".to_string()
    } else {
        format!(
            "This past week I shipped {count} new posts. Here's the one I'd start with:"
        )
    };

    let html = render_email(
        &latest.title,
        &latest.brief,
        &content_html,
        &latest.url,
        &intro,
        &more_posts,
    );
    let subject = format!("Weekly: {}", latest.title);
    let client = reqwest::Client::new();

    for email in &emails {
        let payload = serde_json::json!({
            "from": from_data(),
            "to": [email],
            "subject": subject,
            "html": html,
        });
        let _ = client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&payload)
            .send()
            .await;
    }
}

#[component]
pub fn NewsletterPage(props: &NewsletterSubscription) -> Node {
    rsx! {
        <PageLayout title="Newsletter">
            <div class="py-10 md:py-14 space-y-10">
                {when!(props.email.is_empty() =>
                    <>
                        <section class="page-heading max-w-3xl">
                            <p class="eyebrow">"Field notes"</p>
                            <h1 class="mt-3 text-4xl md:text-5xl font-semibold text-zinc-950">"Newsletter"</h1>
                            <p class="mt-4 text-base text-zinc-600 max-w-2xl leading-relaxed">
                                "A weekly field note from the build process — engineering, product decisions, and systems that hold up under pressure."
                            </p>
                        </section>

                        <form action="/newsletter" method="POST" class="max-w-2xl">
                            <input type="hidden" name="csrf_token" value={props.csrf_token.as_str()} />
                            <input type="hidden" name="csrf_expires" value={props.csrf_expires.as_str()} />
                            <label class="sr-only" for="email">"Email Address"</label>
                            <div class="flex flex-col sm:flex-row gap-3">
                                <input
                                    type="email"
                                    id="email"
                                    name="email"
                                    placeholder="you@domain.com"
                                    required
                                    class="professional-input flex-1"
                                />
                                <button type="submit" class="primary-link whitespace-nowrap">"Subscribe →"</button>
                            </div>
                            <p class="mt-3 text-xs text-zinc-500">"One considered note each week. Unsubscribe anytime."</p>
                        </form>
                    </>
                    else
                    <section class="page-heading max-w-3xl">
                        <p class="eyebrow">"Subscription confirmed"</p>
                        <h2 class="mt-3 text-4xl md:text-5xl font-semibold text-zinc-950">"You're all set."</h2>
                        <p class="mt-4 text-base text-zinc-600 max-w-2xl leading-relaxed">
                            "Thanks for subscribing! Check your email for a confirmation link. Your first newsletter will arrive next week."
                        </p>
                        <div class="mt-7 flex flex-wrap gap-5">
                            <a href="/essays" class="primary-link">"Read latest essays →"</a>
                            <a href="/projects" class="text-link">"View projects →"</a>
                        </div>
                    </section>
                )}
            </div>
        </PageLayout>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers(nonce: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_str(&format!("{CSRF_COOKIE_NAME}={nonce}")).unwrap(),
        );
        headers
    }

    #[test]
    fn accepts_a_valid_signed_newsletter_token() {
        let secret = "test-secret";
        let nonce = "newsletter-nonce";
        let expires = chrono::Utc::now().timestamp() + 60;
        let props = NewsletterSubscription {
            csrf_token: csrf_token(secret, nonce, expires).unwrap(),
            csrf_expires: expires.to_string(),
            ..Default::default()
        };

        assert!(valid_csrf_token_with_secret(
            secret,
            &headers(nonce),
            &props
        ));
    }

    #[test]
    fn rejects_tampered_or_expired_newsletter_tokens() {
        let secret = "test-secret";
        let nonce = "newsletter-nonce";
        let expires = chrono::Utc::now().timestamp() + 60;
        let mut props = NewsletterSubscription {
            csrf_token: csrf_token(secret, nonce, expires).unwrap(),
            csrf_expires: expires.to_string(),
            ..Default::default()
        };

        props.csrf_token.replace_range(..1, "z");
        assert!(!valid_csrf_token_with_secret(
            secret,
            &headers(nonce),
            &props
        ));

        props.csrf_token = csrf_token(secret, nonce, expires).unwrap();
        props.csrf_expires = (chrono::Utc::now().timestamp() - 1).to_string();
        assert!(!valid_csrf_token_with_secret(
            secret,
            &headers(nonce),
            &props
        ));
    }
}
