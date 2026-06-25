use crate::components::PageLayout;
#[cfg(not(target_arch = "wasm32"))]
use axum::extract::Multipart;
use axum::response::{Html, IntoResponse};
use momenta::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NewsletterSubscription {
    pub email: String,
}

impl NewsletterSubscription {
    #[cfg(not(target_arch = "wasm32"))]
    async fn load(mut multipart: Multipart) -> Self {
        let mut email = String::new();

        while let Ok(Some(field)) = multipart.next_field().await {
            if let Some(name) = field.name() {
                if name == "email" {
                    if let Ok(value) = field.text().await {
                        email = value;
                    }
                }
            }
        }

        if !email.is_empty() {
            return NewsletterSubscription { email };
        }
        NewsletterSubscription::default()
    }
}

pub async fn newsletter_get_handler() -> impl IntoResponse {
    let props = NewsletterSubscription::default();
    Html(NewsletterPage::render(&props).to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn newsletter_post_handler(multipart: Multipart) -> impl IntoResponse {
    let props = NewsletterSubscription::load(multipart).await;
    Html(NewsletterPage::render(&props).to_string())
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

                        <form action="/newsletter" method="POST" class="space-y-3 max-w-md" enctype="multipart/form-data">
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
