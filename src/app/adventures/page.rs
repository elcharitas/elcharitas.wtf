use crate::{components::PageLayout, shared::PageParams};
use momenta::prelude::*;

#[component]
pub fn AdventuresPage(_: &PageParams) -> Node {
    rsx! {
        <PageLayout title="Adventures">
            <div class="relative">
                <div class="absolute inset-0 bg-gradient-to-b from-zinc-900/0 via-zinc-900/50 to-zinc-900/100"></div>
                <div class="relative">
                    <h1 class="bg-gradient-to-r from-yellow-500/50 via-yellow-500 to-yellow-500/50 bg-clip-text text-center text-6xl font-bold text-transparent">
                        "Adventures 🌎"
                    </h1>
                    <p class="mx-auto mt-6 max-w-2xl text-center text-lg text-zinc-300">
                        "Join me on my journey around the world, exploring new places, cultures, and experiences."
                    </p>
                </div>
            </div>
            <div class="mt-16 grid grid-cols-1 gap-8 md:grid-cols-2 lg:grid-cols-3">
                <article class="group relative flex flex-col overflow-hidden rounded-lg border border-zinc-800 bg-zinc-900 transition-colors hover:bg-zinc-800/50">
                    <div class="relative aspect-video overflow-hidden">
                        <img
                            src="/images/adventures/placeholder.svg"
                            alt="Adventure Placeholder"
                            class="absolute inset-0 h-full w-full object-cover transition-transform duration-500 group-hover:scale-105"
                        />
                        <div class="absolute inset-0 bg-gradient-to-t from-zinc-900/75 to-zinc-900/0"></div>
                    </div>
                    <div class="flex flex-1 flex-col justify-between p-6">
                        <div class="flex-1">
                            <div class="flex items-center gap-x-4">
                                <time class="text-sm text-zinc-400">"2024"</time>
                                <span class="text-yellow-500">"🌟 New"</span>
                            </div>
                            <div class="mt-4 block">
                                <h2 class="text-xl font-semibold text-zinc-50 group-hover:text-yellow-500/90 transition-colors">
                                    "Coming Soon"
                                </h2>
                                <p class="mt-3 text-sm text-zinc-400 line-clamp-3">
                                    "Stay tuned for exciting adventures and stories from around the globe."
                                </p>
                            </div>
                        </div>
                        <div class="mt-6 flex items-center gap-x-4">
                            <div class="flex items-center gap-x-2">
                                <span class="text-sm text-zinc-400">"Location: "</span>
                                <span class="text-sm text-zinc-300">"Worldwide"</span>
                            </div>
                        </div>
                    </div>
                </article>
            </div>
        </PageLayout>
    }
}
