use crate::{components::AppLayout, shared::PageLoader};
use momenta::prelude::*;
use reqwest::StatusCode;

pub struct ErrorPageProps;

impl PageLoader for ErrorPageProps {
    async fn load(ctx: &mut ngyn::shared::server::NgynContext<'_>) -> Self {
        *ctx.response_mut().status_mut() = StatusCode::NOT_FOUND;

        ErrorPageProps {}
    }
}

#[component]
pub fn ErrorPage(_: &ErrorPageProps) -> Node {
    rsx! {
        <AppLayout title="Page Not Found - 404">
            <div class="min-h-screen bg-zinc-800 bg-opacity-30 flex items-center justify-center p-4 sm:p-6 lg:p-8">
                <div class="max-w-lg w-full space-y-8 p-8 rounded-xl shadow-xl">
                    <div class="text-center space-y-4">
                        <div class="relative mx-auto w-24 h-24">
                            <svg
                                class="absolute inset-0 w-full h-full text-zinc-500 animate-pulse"
                                fill="none"
                                stroke="currentColor"
                                stroke_linecap="round"
                                stroke_linejoin="round"
                                stroke_width="1.5"
                                viewBox="0 0 24 24"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <circle cx="12" cy="12" r="10" class="animate-ping"></circle>
                                <line x1="12" x2="12" y1="8" y2="12"></line>
                                <line x1="12" x2="12.01" y1="16" y2="16"></line>
                            </svg>
                        </div>
                        <h2 class="text-4xl font-bold text-zinc-100 tracking-tight">
                            "Oops! Something's missing"
                        </h2>
                        <p class="text-lg text-zinc-300">
                            "This page could not be found or has been relocated"
                        </p>
                    </div>
                    <div class="flex justify-center space-x-4">
                        <a
                            href="/"
                            class="transform transition-all duration-200 px-6 py-3 text-base font-medium rounded-lg text-white bg-zinc-700 hover:bg-zinc-600 hover:scale-105 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-zinc-500 focus:ring-offset-zinc-900"
                        >
                            "Go Home"
                        </a>
                        <a
                            href="/adventures"
                            class="transform transition-all duration-200 px-6 py-3 text-base font-medium rounded-lg text-zinc-300 border border-zinc-600 hover:bg-zinc-700 hover:scale-105 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-zinc-500 focus:ring-offset-zinc-900"
                        >
                            "View Timeline"
                        </a>
                    </div>
                </div>
            </div>
        </AppLayout>
    }
}
