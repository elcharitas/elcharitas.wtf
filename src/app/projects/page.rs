use crate::components::AppLayout;
use momenta::prelude::*;
use ngyn::prelude::*;

#[derive(Param)]
pub struct PageParams;

#[component]
pub fn ProjectsPage(_: &PageParams) -> Node {
    rsx! {
        <AppLayout title="Projects">
            <div class="bg-zinc-800 min-h-screen">
                <div class="container mx-auto px-4 py-12">
                    <h1 class="text-4xl font-bold text-zinc-50 mb-8">"Projects 🚀"</h1>
                    <p class="text-lg text-zinc-200 mb-12">
                        "Here are some of the projects I've worked on, from open source contributions to personal experiments."
                    </p>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        <article class="bg-zinc-900 rounded-lg overflow-hidden shadow-lg hover:scale-105 transition-transform duration-300">
                            <div class="p-6">
                                <h2 class="text-xl font-semibold text-zinc-50 mb-2">"Chakra UI (Svelte)"</h2>
                                <p class="text-zinc-300 mb-4">"A port of Chakra UI components for Svelte applications, bringing the power of Chakra's design system to Svelte."</p>
                                <div class="flex items-center justify-between text-zinc-400">
                                    <span>"Svelte"</span>
                                    <a href="https://github.com/chakra-ui/svelte" class="text-yellow-500 hover:text-yellow-400">"View Project →"</a>
                                </div>
                            </div>
                        </article>
                        <article class="bg-zinc-900 rounded-lg overflow-hidden shadow-lg hover:scale-105 transition-transform duration-300">
                            <div class="p-6">
                                <h2 class="text-xl font-semibold text-zinc-50 mb-2">"Ngyn"</h2>
                                <p class="text-zinc-300 mb-4">"A modern Rust framework for building fast, reliable web applications with an intuitive API design."</p>
                                <div class="flex items-center justify-between text-zinc-400">
                                    <span>"Rust"</span>
                                    <a href="https://crates.io/crates/ngyn" class="text-yellow-500 hover:text-yellow-400">"View Project →"</a>
                                </div>
                            </div>
                        </article>
                        <article class="bg-zinc-900 rounded-lg overflow-hidden shadow-lg hover:scale-105 transition-transform duration-300">
                            <div class="p-6">
                                <h2 class="text-xl font-semibold text-zinc-50 mb-2">"More Coming Soon"</h2>
                                <p class="text-zinc-300 mb-4">"Stay tuned for more exciting projects and contributions to the open source community."</p>
                                <div class="flex items-center justify-between text-zinc-400">
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
