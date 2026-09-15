use crate::shared::*;
use momenta::prelude::*;

fn format_published_date(published_at: &str) -> String {
    chrono::NaiveDate::parse_from_str(published_at, "%Y-%m-%d")
        .map(|date| date.format("%b %d, %Y").to_string())
        .unwrap_or_else(|_| published_at.to_string())
}

pub struct ArticleProps {
    pub post: Post,
    pub show_read_more: bool,
}

#[component]
pub fn Article(
    ArticleProps {
        post,
        show_read_more,
    }: &ArticleProps,
) -> Node {
    let category = post.tags.first();
    rsx! {
        <a
            href={format!("/essays/{}", post.slug)}
            class="group flex flex-col h-full border-t border-zinc-900/15 py-5"
        >
            <div class="flex flex-col gap-3 flex-1">
                <div class="flex items-center justify-between gap-2">
                    <span class="eyebrow">
                        {category.map_or("general", |c| &c.name)}
                    </span>
                    {when!(let Some(published_at) = &post.published_at =>
                        <time
                            datetime={published_at}
                            class="text-xs text-zinc-500"
                        >
                            <span>
                                {format_published_date(published_at)}
                            </span>
                        </time>
                    )}
                </div>

                <h2 class="text-xl font-semibold text-zinc-900 group-hover:text-[var(--accent-strong)] leading-snug transition-colors">
                    {&post.title}
                </h2>

                <p class="text-sm text-zinc-600 leading-relaxed line-clamp-3">
                    {&post.brief[0..(post.brief.len().min(120))]}...
                </p>

                <div class="flex items-center justify-between mt-auto">
                    <span class="text-xs text-zinc-500">{format!("{} views", post.views.unwrap_or(0))}</span>
                    {when!(show_read_more =>
                        <span class="text-link text-xs">"Read essay →"</span>
                    )}
                </div>
            </div>
        </a>
    }
}

#[component]
pub fn ProjectArticle(project: &Project) -> Node {
    let brief = if project.description.is_empty() {
        &project.name
    } else {
        &project.description
    };
    rsx! {
        <article class="group flex flex-col h-full border-t border-zinc-900/15 py-5">
            <div class="flex flex-col gap-3 flex-1">
                <h2 class="text-xl font-semibold text-zinc-900">
                    {&project.name}
                </h2>
                <p class="text-sm text-zinc-600 leading-relaxed">
                    {&brief[0..(brief.len().min(120))]}...
                </p>
                {if !project.tags.is_empty() {
                    rsx! {
                        <div class="flex flex-wrap gap-x-4 gap-y-1">
                            {project.tags.iter().take(4).map(|tag| rsx! {
                                <span class="meta-label">{tag.replace('-', " ")}</span>
                            })}
                        </div>
                    }
                } else {
                    rsx! { <></> }
                }}
                <div class="flex items-center justify-between flex-wrap gap-2 mt-auto">
                    <span class="flex items-center gap-3 text-xs text-zinc-500">
                        <span>{format!("{} stars", project.stargazers_count)}</span>
                        {when!(let Some(lang) = &project.language =>
                            <span>{lang}</span>
                        )}
                    </span>
                    <a href={&project.url} class="text-link text-xs">"View project ↗"</a>
                </div>
            </div>
        </article>
    }
}
