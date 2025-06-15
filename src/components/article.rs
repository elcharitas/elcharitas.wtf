use crate::{components::card::Card, shared::*};
use momenta::prelude::*;

pub struct ArticleProps {
    pub post: Post<u64>,
    pub show_read_more: bool,
}

#[component]
pub fn Article(
    ArticleProps {
        post,
        show_read_more,
    }: &ArticleProps,
) -> Node {
    rsx! {
        <Card>
            <a href={format!("/{}/{}", post.category, post.slug)}>
                <div class="p-4 md:p-8">
                    <div class="flex justify-between gap-2 items-center mb-2">
                    <span class="text-xs duration-1000 text-zinc-200 group-hover:text-white group-hover:border-zinc-200 drop-shadow-orange">
                        {when!(post.date.is_some() =>
                            <time datetime={post.date.as_deref().unwrap_or_default()}>
                                {chrono::NaiveDate::parse_from_str(post.date.as_ref().unwrap().as_str(), "%Y-%m-%d").unwrap().format("%B %d, %Y").to_string()}
                            </time>
                        )}
                    </span>
                    <span class="text-zinc-400 text-xs flex items-center gap-1">
                        {when!(post.comments.is_some() =>
                            <span>
                                <!-- FontAwesomeIcon class="w-4 h-4 text-yellow-200/60" icon="message-square" -->
                                {post.comments}
                            </span>
                        )}
                        {when!(post.category == "projects" =>
                            <div class="w-4 h-4 text-yellow-200/60 ml-3" />
                        else
                            <div class="w-4 h-4 text-yellow-200/60 ml-3" />
                        )}
                        {post.views}
                    </span>
                    </div>
                    <h2 class="z-20 text-xl font-medium duration-1000 lg:text-3xl text-zinc-200 group-hover:text-white font-display">
                        {&post.title}
                    </h2>
                    <p class="z-20 mt-4 mb-8 text-sm  duration-1000 text-zinc-400 group-hover:text-zinc-200">
                        {&post.brief[0..120]}...
                    </p>
                    {when!(show_read_more =>
                        <div class="absolute bottom-4 md:bottom-8">
                            <p class="hidden text-zinc-200 hover:text-zinc-50 lg:block">
                                "Read more" <span aria_hidden>&rarr;</span>
                            </p>
                        </div>
                    )}
                </div>
            </a>
        </Card>
    }
}

#[component]
pub fn ProjectArticle(props: &Project) -> Node {
    rsx! {
        <article class="bg-zinc-900 rounded-lg overflow-hidden shadow-lg hover:scale-105 transition-transform duration-300">
            <div class="p-6">
                <h2 class="text-xl font-semibold text-zinc-50 mb-2">{&props.name}</h2>
                <p class="text-zinc-300 mb-4">
                    {&props.description}
                </p>
                <div class="flex items-center justify-between text-zinc-400">
                    <span>{&props.tags.join(", ")}</span>
                    <a href="https://github.com/chakra-ui/svelte" class="text-yellow-500 hover:text-yellow-400">"View Project →"</a>
                </div>
            </div>
        </article>
    }
}
