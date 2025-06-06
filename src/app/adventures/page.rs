use crate::components::AppLayout;
use momenta::prelude::*;
use ngyn::prelude::*;

#[derive(Param)]
pub struct PageParams;

#[component]
pub fn AdventuresPage(_: &PageParams) -> Node {
    rsx! {
        <AppLayout title="Adventures">
            <div class="bg-zinc-800 min-h-screen">
                <div class="container mx-auto px-4 py-12">
                    <h1 class="text-4xl font-bold text-zinc-50 mb-8">"Adventures 🌎"</h1>
                    <p class="text-lg text-zinc-200 mb-12">
                        "Join me on my journey around the world, exploring new places, cultures, and experiences."
                    </p>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                        <article class="bg-zinc-900 rounded-lg overflow-hidden shadow-lg hover:scale-105 transition-transform duration-300">
                            <img src="/images/adventures/placeholder.svg" alt="Adventure Placeholder" class="w-full h-48 object-cover" />
                            <div class="p-6">
                                <h2 class="text-xl font-semibold text-zinc-50 mb-2">"Coming Soon"</h2>
                                <p class="text-zinc-300">"Stay tuned for exciting adventures and stories from around the globe."</p>
                                <div class="mt-4 flex items-center justify-between text-zinc-400">
                                    <span>"2024"</span>
                                    <span>"🌟 New"</span>
                                </div>
                            </div>
                        </article>
                    </div>
                </div>
            </div>
        </AppLayout>
    }
}
