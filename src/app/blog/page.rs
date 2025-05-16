use crate::components::AppLayout;
use crate::shared::*;
use serde::{Deserialize, Serialize};
use simple_rsx::*;

#[derive(Serialize, Deserialize)]
pub struct BlogProps {
    pub initial_posts: Vec<Post<u64>>,
    pub initial_cursor: Option<String>,
}

#[component]
pub fn Blog() -> Node {
    let blank = String::new();
    rsx! {
        <AppLayout title="Writings">
             <div class="bg-zinc-800 min-h-screen">
                <div class="container mx-auto px-4 py-12">
                    <h1 class="text-4xl font-bold text-zinc-50 mb-8">"Blogs ✍🏼"</h1>
                    <p class="text-lg text-zinc-200 mb-12">
                        "I write about my experiences and thoughts on how to do software development, productivity, and life."
                    </p>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-12">
                        {Vec::<Post<u64>>::new().iter().map(|post| {
                            rsx! {
                                <article class="bg-zinc-900 rounded-lg overflow-hidden shadow-lg">
                                    <img src={post.cover_image.as_ref().unwrap_or(&blank)} alt={&post.title} class="w-full h-48 object-cover" />
                                    <div class="p-6">
                                        <h2 class="text-xl font-semibold text-zinc-50 mb-2">{&post.title}</h2>
                                        <p class="text-zinc-300">{&post.brief}</p>
                                        <div class="mt-4 flex items-center justify-between text-zinc-400">
                                            <span>{post.date.as_ref().unwrap_or(&String::new())}</span>
                                            <span>{post.views.unwrap_or(0)} "views"</span>
                                        </div>
                                    </div>
                                </article>
                            }
                        })}
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                        {Vec::<Post<u64>>::new().iter().map(|post| {
                            rsx! {
                                <article class="bg-zinc-900 rounded-lg overflow-hidden shadow-lg">
                                    <div class="p-6">
                                        <h2 class="text-xl font-semibold text-zinc-50 mb-2">{&post.title}</h2>
                                        <p class="text-zinc-300">{&post.brief}</p>
                                        <div class="mt-4 flex items-center justify-between text-zinc-400">
                                            <span>{post.date.as_ref().unwrap_or(&String::new())}</span>
                                            <span>{post.views.unwrap_or(0)} "views"</span>
                                        </div>
                                    </div>
                                </article>
                            }
                        })}
                    </div>
                </div>
            </div>
        </AppLayout>
    }
}
