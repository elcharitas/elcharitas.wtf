use serde::{Deserialize, Serialize};
use simple_rsx::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsletterSubscription {
    pub email: String,
    pub publication_id: String,
}

#[component]
pub fn NewsletterPage() -> Node {
    rsx! {
        <div class="h-[65vh] flex flex-col justify-center">
            <div class="w-full max-w-md mx-auto space-y-4">
                <div class="space-y-2">
                    <h1 class="text-3xl font-bold text-center text-white">
                        "Subscribe to my newsletter 🔥"
                    </h1>
                    <p class="text-zinc-500 dark:text-zinc-400 text-center">
                        Get the latest updates and news delivered to your inbox.
                    </p>
                </div>
                <form action="/api/newsletter/subscribe" method="POST">
                    <div class="flex space-x-2 mb-4">
                        <label class="sr-only" for_="email">
                            Email
                        </label>
                        <input
                            class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                            id="email"
                            name="email"
                            placeholder="mail@example.com"
                            required
                            type_="email"
                            // data-tour="email-form"
                            // data-tour-title="Subscribe to my newsletter"
                            // data-tour-description="We'll send you an email to confirm your subscription."
                            // data-tour-position="right"
                        />
                        <button
                            type_="submit"
                            class="flex items-center justify-center h-10 px-4 text-sm font-medium rounded-md border border-transparent bg-zinc-700 hover:bg-zinc-600 text-white disabled:cursor-not-allowed disabled:opacity-50"
                            // data-tour="Subscribe"
                            // data-tour-description="Click here to subscribe to my newsletter."
                            // data-tour-position="right"
                        >
                            Subscribe
                        </button>
                    </div>
                    <p class="text-xs text-zinc-500 dark:text-zinc-400 text-center">
                        By subscribing, you agree to our
                        <a class="underline underline-offset-2" href="#">
                            Privacy Policy
                        </a>
                    </p>
                </form>
            </div>
        </div>
    }
}
