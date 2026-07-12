use crate::components::PageLayout;
use axum::response::{Html, IntoResponse};
use momenta::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NewsletterSubscription {
    pub email: String,
}

impl NewsletterSubscription {
    fn from_body(body: &str) -> Self {
        let email = url::form_urlencoded::parse(body.as_bytes())
            .find(|(k, _)| k == "email")
            .map(|(_, v)| v.to_string())
            .unwrap_or_default();
        NewsletterSubscription { email }
    }
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

pub async fn newsletter_get_handler() -> impl IntoResponse {
    let props = NewsletterSubscription::default();
    Html(NewsletterPage::render(&props).to_string())
}

pub async fn newsletter_post_handler(body: String) -> impl IntoResponse {
    let props = NewsletterSubscription::from_body(&body);

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

    Html(NewsletterPage::render(&props).to_string())
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
            <div class="py-4 md:py-8 space-y-8">
                {when!(props.email.is_empty() =>
                    <>
                        <section class="space-y-4">
                            <h1 class="text-4xl md:text-5xl font-semibold text-white">"Newsletter"</h1>
                            <div class="section-rule"></div>
                            <p class="text-base text-zinc-300 max-w-2xl">
                                "A weekly field note from the build process — engineering, product decisions, and systems that hold up under pressure."
                            </p>
                        </section>

                        <form action="/newsletter" method="POST" class="space-y-3 max-w-md">
                            <label class="sr-only" for="email">"Email Address"</label>
                            <input
                                type="email"
                                id="email"
                                name="email"
                                placeholder="you@domain.com"
                                required
                                class="w-full h-12 px-4 rounded-lg border border-zinc-700 bg-zinc-900/50 text-white placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-zinc-500/50 focus:border-zinc-500 transition-colors"
                            />
                            <button
                                type="submit"
                                class="btn-accent w-full h-12 text-sm font-semibold rounded-lg"
                            >
                                "Subscribe for free"
                            </button>
                            <p class="text-xs text-zinc-500">"No spam. Unsubscribe anytime."</p>
                        </form>
                    </>
                    else
                    <section class="space-y-4">
                        <h2 class="text-3xl font-semibold text-white">"You're All Set"</h2>
                        <div class="section-rule"></div>
                        <p class="text-base text-zinc-300 max-w-2xl">
                            "Thanks for subscribing! Check your email for a confirmation link. Your first newsletter will arrive next week."
                        </p>
                        <div class="flex flex-wrap gap-3 pt-2">
                            <a href="/essays" class="btn-accent px-5 py-3 text-sm font-semibold rounded-md">"Read Latest Posts"</a>
                            <a href="/projects" class="btn-ghost px-5 py-3 text-sm font-semibold rounded-md">"View Projects"</a>
                        </div>
                    </section>
                )}
            </div>
        </PageLayout>
    }
}
