use crate::{
    components::PageLayout,
    shared::{PageLoader, HASHNODE_CLIENT, NEWSLETTER_MUTATION},
};
use momenta::prelude::*;
use ngyn::{
    prelude::*,
    shared::server::{transformer::FormData, Transformer},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NewsletterSubscription {
    pub email: String,
}

impl PageLoader for NewsletterSubscription {
    async fn load(ctx: &mut NgynContext<'_>) -> Self {
        let body = Body::transform(ctx);
        if let Ok(multipart) = body.to_multipart() {
            let form_data = FormData::from_multipart(multipart)
                .await
                .unwrap_or_default();
            let email = form_data.get::<String>("email").unwrap_or_default();

            let variables = serde_json::json!({
                "input": {
                    "email": email,
                    "publicationId": "6231526bc4a093f00c8acd3b"
                }
            });

            if let Ok(_) = HASHNODE_CLIENT
                .execute_query::<Value>(NEWSLETTER_MUTATION.to_owned(), Some(variables))
                .await
            {
                return NewsletterSubscription { email };
            }
        }
        NewsletterSubscription::default()
    }
}

#[component]
pub fn NewsletterPage(props: &NewsletterSubscription) -> Node {
    rsx! {
        <PageLayout title="Subscribe to my Newsletter">
            <div class="min-h-[80vh] flex flex-col justify-center py-12">
                <div class="w-full max-w-2xl mx-auto space-y-8">
                    {when!(props.email.is_empty() =>
                        <>
                            <div class="text-center space-y-6">
                                <div class="inline-flex items-center px-4 py-2 rounded-full bg-gradient-to-r from-yellow-500/10 to-orange-500/10 border border-yellow-500/20 mb-4">
                                    <i class="fas fa-envelope text-yellow-400 mr-2"></i>
                                    <span class="text-sm font-medium text-yellow-300">"Weekly Newsletter"</span>
                                </div>

                                <h1 class="text-4xl md:text-6xl font-bold text-center">
                                    <span class="text-gradient">
                                        "Stay in the Loop"
                                        <span class="inline-block ml-2 animate-bounce">"🚀"</span>
                                    </span>
                                </h1>

                                <p class="text-lg md:text-xl text-zinc-300 max-w-xl mx-auto leading-relaxed">
                                    "Get exclusive insights on software engineering, web development, and the latest tech trends. "
                                </p>
                            </div>
                            <div class="grid md:grid-cols-3 gap-6 my-12">
                                <div class="text-center p-6 rounded-xl bg-zinc-900/50 border border-zinc-800/50 hover-lift">
                                    <div class="w-12 h-12 mx-auto mb-4 rounded-full bg-gradient-to-r from-blue-500/20 to-cyan-500/20 flex items-center justify-center">
                                        <i class="fas fa-code text-blue-400 text-xl"></i>
                                    </div>
                                    <h3 class="font-semibold text-white mb-2">"Code Deep Dives"</h3>
                                    <p class="text-sm text-zinc-400">"In-depth tutorials and code reviews from real projects"</p>
                                </div>

                                <div class="text-center p-6 rounded-xl bg-zinc-900/50 border border-zinc-800/50 hover-lift">
                                    <div class="w-12 h-12 mx-auto mb-4 rounded-full bg-gradient-to-r from-green-500/20 to-emerald-500/20 flex items-center justify-center">
                                        <i class="fas fa-lightbulb text-green-400 text-xl"></i>
                                    </div>
                                    <h3 class="font-semibold text-white mb-2">"Tech Insights"</h3>
                                    <p class="text-sm text-zinc-400">"Latest trends and tools that actually matter"</p>
                                </div>

                                <div class="text-center p-6 rounded-xl bg-zinc-900/50 border border-zinc-800/50 hover-lift">
                                    <div class="w-12 h-12 mx-auto mb-4 rounded-full bg-gradient-to-r from-purple-500/20 to-pink-500/20 flex items-center justify-center">
                                        <i class="fas fa-rocket text-purple-400 text-xl"></i>
                                    </div>
                                    <h3 class="font-semibold text-white mb-2">"Career Tips"</h3>
                                    <p class="text-sm text-zinc-400">"Practical advice for growing as a developer"</p>
                                </div>
                            </div>
                            <div class="max-w-md mx-auto">
                                <form action="/newsletter" method="POST" class="space-y-4" enctype="multipart/form-data">
                                    <div class="relative group">
                                        <label class="sr-only" for="email">"Email Address"</label>
                                        <div class="relative">
                                            <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                                                <i class="fas fa-envelope text-zinc-400 group-focus-within:text-yellow-400 transition-colors"></i>
                                            </div>
                                            <input
                                                type="email"
                                                class="w-full h-14 pl-12 pr-4 rounded-xl border border-zinc-700 bg-zinc-900/50 backdrop-blur-sm text-white placeholder-zinc-400 focus:outline-none focus:ring-2 focus:ring-yellow-500/50 focus:border-yellow-500/50 transition-all duration-300"
                                                id="email"
                                                name="email"
                                                placeholder="yourname@email.com"
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
                                        class="w-full h-14 gradient-border text-white font-semibold rounded-xl hover-lift transition-all duration-300 group relative overflow-hidden"
                                        data_tour="Subscribe"
                                        data_tour_description="Click here to subscribe to my newsletter."
                                        data_tour_position="right"
                                    >
                                        <span class="relative z-10 flex items-center justify-center space-x-2">
                                            <i class="fas fa-paper-plane group-hover:translate-x-1 transition-transform duration-300"></i>
                                            <span>"Subscribe for Free"</span>
                                        </span>
                                        <div class="absolute inset-0 bg-gradient-to-r from-yellow-500/20 to-orange-500/20 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                                    </button>

                                    <div class="flex items-center justify-center space-x-6 text-xs text-zinc-500">
                                        <div class="flex items-center space-x-1">
                                            <i class="fas fa-check text-green-400"></i>
                                            <span>"No spam, ever"</span>
                                        </div>
                                        <div class="flex items-center space-x-1">
                                            <i class="fas fa-check text-green-400"></i>
                                            <span>"Unsubscribe anytime"</span>
                                        </div>
                                        <div class="flex items-center space-x-1">
                                            <i class="fas fa-check text-green-400"></i>
                                            <span>"Weekly updates"</span>
                                        </div>
                                    </div>

                                    <p class="text-xs text-zinc-500 text-center leading-relaxed">
                                        "By subscribing, you agree to receive email updates. You can unsubscribe at any time. "
                                        <a class="text-yellow-400 hover:text-yellow-300 underline underline-offset-2 transition-colors" href="#">
                                            "Privacy Policy"
                                        </a>
                                    </p>
                                </form>
                            </div>
                        </>
                        else
                        <div class="max-w-md mx-auto text-center space-y-6">
                            <div class="w-20 h-20 mx-auto rounded-full bg-gradient-to-r from-green-500/20 to-emerald-500/20 flex items-center justify-center mb-6">
                                <i class="fas fa-check text-green-400 text-3xl"></i>
                            </div>
                            <h2 class="text-2xl font-bold text-white">"You're All Set! 🎉"</h2>
                            <p class="text-zinc-300">
                                "Thanks for subscribing! Check your email for a confirmation link. "
                                "Your first newsletter will arrive next week."
                            </p>
                            <div class="flex justify-center space-x-4 pt-4">
                                <a href="/essays" class="px-6 py-3 rounded-lg bg-zinc-800 hover:bg-zinc-700 text-white transition-colors hover-lift">
                                    "Read Latest Posts"
                                </a>
                                <a href="/projects" class="px-6 py-3 rounded-lg border border-zinc-700 hover:border-zinc-600 text-white transition-colors hover-lift">
                                    "View Projects"
                                </a>
                            </div>
                        </div>
                    )}
                </div>
            </div>
        </PageLayout>
    }
}
