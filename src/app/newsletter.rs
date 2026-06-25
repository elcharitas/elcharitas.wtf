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
        <PageLayout title="Subscribe to my Newsletter">
            <div class="py-6 md:py-10 space-y-8">
                <div class="w-full space-y-8">
                    {when!(props.email.is_empty() =>
                        <>
                            <section class="glass-panel rounded-3xl p-6 md:p-8 lg:p-10">
                                <div class="grid grid-cols-1 lg:grid-cols-[1fr_0.9fr] gap-8 items-start">
                                    <div class="space-y-5">
                                        <div class="inline-flex items-center px-3 py-1.5 rounded-md bg-zinc-900 border border-zinc-700">
                                            <i class="fas fa-envelope text-zinc-300 mr-2"></i>
                                            <span class="text-xs uppercase tracking-[0.1em] font-semibold text-zinc-300">"Weekly Newsletter"</span>
                                        </div>

                                        <h1 class="text-4xl md:text-6xl font-semibold text-white leading-tight">"A weekly field note from the build process."</h1>

                                        <p class="text-lg text-zinc-300 max-w-2xl leading-relaxed">
                                            "Get practical breakdowns on software engineering, product decisions, and systems that hold up under pressure."
                                        </p>

                                        <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 text-sm">
                                            <div class="border border-zinc-800 rounded-xl p-3 bg-zinc-950/60">
                                                <p class="text-zinc-500 text-xs uppercase tracking-[0.08em]">"Frequency"</p>
                                                <p class="text-zinc-100 mt-1 font-semibold">"Weekly"</p>
                                            </div>
                                            <div class="border border-zinc-800 rounded-xl p-3 bg-zinc-950/60">
                                                <p class="text-zinc-500 text-xs uppercase tracking-[0.08em]">"Style"</p>
                                                <p class="text-zinc-100 mt-1 font-semibold">"No fluff"</p>
                                            </div>
                                            <div class="border border-zinc-800 rounded-xl p-3 bg-zinc-950/60">
                                                <p class="text-zinc-500 text-xs uppercase tracking-[0.08em]">"Focus"</p>
                                                <p class="text-zinc-100 mt-1 font-semibold">"Engineering + Product"</p>
                                            </div>
                                        </div>
                                    </div>

                                    <aside class="rounded-2xl border border-zinc-800 bg-zinc-950/70 p-5 space-y-4">
                                        <p class="text-xs uppercase tracking-[0.1em] text-zinc-500">"Issue Preview"</p>
                                        <h2 class="text-xl font-semibold text-white">"Inside a typical issue"</h2>
                                        <ul class="space-y-3 text-sm text-zinc-300">
                                            <li class="flex items-start gap-2"><i class="fas fa-caret-right text-zinc-500 mt-1"></i><span>"One architecture lesson from a real implementation."</span></li>
                                            <li class="flex items-start gap-2"><i class="fas fa-caret-right text-zinc-500 mt-1"></i><span>"A short teardown of a product UX decision."</span></li>
                                            <li class="flex items-start gap-2"><i class="fas fa-caret-right text-zinc-500 mt-1"></i><span>"A practical next step you can apply immediately."</span></li>
                                        </ul>
                                    </aside>
                                </div>
                            </section>

                            <section class="grid grid-cols-1 xl:grid-cols-[1fr_340px] gap-6 items-start">
                                <div class="glass-panel rounded-3xl p-6 md:p-8">
                                    <h3 class="text-2xl font-semibold text-white mb-4">"Join the list"</h3>
                                    <form action="/newsletter" method="POST" class="space-y-4" enctype="multipart/form-data">
                                        <div class="relative group">
                                            <label class="sr-only" for="email">"Email Address"</label>
                                            <div class="relative">
                                                <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                                                    <i class="fas fa-envelope text-zinc-400 group-focus-within:text-zinc-200 transition-colors"></i>
                                                </div>
                                                <input
                                                    type="email"
                                                    class="w-full h-14 pl-12 pr-4 rounded-xl border border-zinc-700 bg-zinc-900/50 text-white placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-zinc-500/50 focus:border-zinc-500 transition-all duration-300"
                                                    id="email"
                                                    name="email"
                                                    placeholder="you@domain.com"
                                                    required
                                                    data_tour="email-form"
                                                    data_tour_title="Subscribe to my newsletter"
                                                    data_tour_description="We'll send you an email to confirm your subscription."
                                                    data_tour_position="right"
                                                />
                                            </div>
                                        </div>

                                        <button
                                            type="submit"
                                            class="w-full h-14 bg-zinc-900 border border-zinc-700 text-white font-semibold rounded-xl soft-lift hover:bg-zinc-800 transition-all duration-300"
                                            data_tour="Subscribe"
                                            data_tour_description="Click here to subscribe to my newsletter."
                                            data_tour_position="right"
                                        >
                                            <span class="flex items-center justify-center gap-2">
                                                <i class="fas fa-paper-plane"></i>
                                                <span>"Subscribe for free"</span>
                                            </span>
                                        </button>

                                        <div class="grid grid-cols-1 sm:grid-cols-3 gap-2 text-xs text-zinc-500">
                                            <div class="flex items-center gap-1"><i class="fas fa-check text-zinc-300"></i><span>"No spam"</span></div>
                                            <div class="flex items-center gap-1"><i class="fas fa-check text-zinc-300"></i><span>"Unsubscribe anytime"</span></div>
                                            <div class="flex items-center gap-1"><i class="fas fa-check text-zinc-300"></i><span>"Weekly"</span></div>
                                        </div>

                                        <p class="text-xs text-zinc-500 leading-relaxed">
                                            "By subscribing, you agree to receive email updates. You can unsubscribe at any time. "
                                            <a class="text-zinc-300 hover:text-white underline underline-offset-2 transition-colors" href="#">
                                                "Privacy Policy"
                                            </a>
                                        </p>
                                    </form>
                                </div>

                                <aside class="glass-panel rounded-3xl p-5 space-y-4">
                                    <p class="text-xs uppercase tracking-[0.1em] text-zinc-500">"What readers get"</p>
                                    <div class="space-y-3 text-sm text-zinc-300">
                                        <div class="border border-zinc-800 rounded-xl p-3 bg-zinc-950/60">
                                            <p class="font-semibold text-zinc-100 mb-1">"Code Deep Dives"</p>
                                            <p class="text-zinc-400">"Tactical breakdowns from real projects."</p>
                                        </div>
                                        <div class="border border-zinc-800 rounded-xl p-3 bg-zinc-950/60">
                                            <p class="font-semibold text-zinc-100 mb-1">"Tech Insights"</p>
                                            <p class="text-zinc-400">"Signal over noise on tools and trends."</p>
                                        </div>
                                        <div class="border border-zinc-800 rounded-xl p-3 bg-zinc-950/60">
                                            <p class="font-semibold text-zinc-100 mb-1">"Career Lessons"</p>
                                            <p class="text-zinc-400">"Practical advice for long-term growth."</p>
                                        </div>
                                    </div>
                                </aside>
                            </section>
                        </>
                        else
                        <section class="max-w-2xl mx-auto glass-panel rounded-3xl p-8 md:p-10 text-center space-y-6">
                            <div class="w-20 h-20 mx-auto rounded-2xl bg-zinc-900/70 border border-zinc-700 flex items-center justify-center mb-2">
                                <i class="fas fa-check text-zinc-200 text-3xl"></i>
                            </div>
                            <h2 class="text-3xl font-semibold text-white">"You're All Set"</h2>
                            <p class="text-zinc-300 leading-relaxed">
                                "Thanks for subscribing! Check your email for a confirmation link. "
                                "Your first newsletter will arrive next week."
                            </p>
                            <div class="flex justify-center flex-wrap gap-3 pt-2">
                                <a href="/essays" class="px-6 py-3 rounded-lg bg-zinc-800 hover:bg-zinc-700 text-white transition-colors soft-lift">
                                    "Read Latest Posts"
                                </a>
                                <a href="/projects" class="px-6 py-3 rounded-lg border border-zinc-700 hover:border-zinc-600 text-white transition-colors soft-lift">
                                    "View Projects"
                                </a>
                            </div>
                        </section>
                    )}
                </div>
            </div>
        </PageLayout>
    }
}
