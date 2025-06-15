use crate::{
    components::{article::ProjectArticle, PageLayout},
    shared::PageParams,
};
use momenta::prelude::*;

#[component]
pub fn ProjectsPage(_: &PageParams) -> Node {
    rsx! {
        <PageLayout title="Projects">
            <div class="container mx-auto px-4 py-12">
                <h1 class="text-4xl font-bold text-zinc-50 mb-8">"Projects 🚀"</h1>
                <p class="text-lg text-zinc-200 mb-12">
                    "Here are some of the projects I've worked on, from open source contributions to personal experiments."
                </p>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    <ProjectArticle name="Chakra UI (Svelte)" description="A port of Chakra UI components for Svelte applications, bringing the power of Chakra's design system to Svelte." url={"https://github.com/chakra-ui/svelte".to_string()} image={"https://chakra-ui-svelte-docs.netlify.app/chakra-ui-svelte.png".to_string()} tags={vec!["Svelte".to_string()]} />
                    <ProjectArticle name="More Coming Soon" description="Stay tuned for more exciting projects and contributions to the open source community." url={"https://github.com/chakra-ui/svelte".to_string()} image={"https://chakra-ui-svelte-docs.netlify.app/chakra-ui-svelte.png".to_string()} tags={vec!["2024".to_string(), "🌟 New".to_string()]} />
                    <ProjectArticle name="Elcharitas" description="A simple and lightweight blog platform, built with Rust and SvelteKit." url={"https://github.com/elcharitas/elcharitas".to_string()} image={"https://elcharitas.wtf/og.png".to_string()} tags={vec!["Rust".to_string(), "SvelteKit".to_string()]} />
                </div>
            </div>
        </PageLayout>
    }
}
