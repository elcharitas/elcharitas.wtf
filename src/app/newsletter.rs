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
fn render_email(title: &str, brief: &str, url: &str) -> String {
    let template = include_str!("../emails/newsletter.mrml")
        .replace("{{subject}}", &format!("Weekly: {title}"))
        .replace("{{title}}", title)
        .replace("{{brief}}", brief)
        .replace("{{url}}", url);

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
    let Some(latest) = posts.first() else {
        return;
    };

    let html = render_email(&latest.title, &latest.brief, &latest.url);
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
