use crate::shared::*;
use momenta::prelude::*;

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
            class="group flex flex-col h-full card-item rounded-lg p-5 transition-all"
        >
            <div class="flex flex-col gap-4 flex-1">
                <div class="flex items-center justify-between gap-2">
                    <span class="inline-flex items-center px-2.5 py-1 text-[11px] uppercase tracking-[0.08em] font-semibold rounded-md" style="background: var(--accent-dim); border: 1px solid var(--accent-border); color: var(--accent);">
                        {category.map_or("general", |c| &c.name)}
                    </span>
                    {when!(let Some(published_at) = &post.published_at =>
                        <time
                            datetime={&*published_at}
                            class="text-xs text-zinc-500"
                        >
                            <span>
                                {chrono::DateTime::parse_from_rfc3339(&published_at).unwrap_or_default().format("%b %d, %Y").to_string()}
                            </span>
                        </time>
                    )}
                </div>

                <h2 class="text-xl md:text-2xl font-semibold text-zinc-100 group-hover:text-white leading-tight">
                    {&post.title}
                </h2>

                <p class="text-sm md:text-base text-zinc-400 leading-relaxed line-clamp-3">
                    {&post.brief[0..(post.brief.len().min(120))]}...
                </p>

                <div class="pt-2 flex items-center justify-between mt-auto">
                    <span class="text-xs text-zinc-500">{format!("{} views", post.views.unwrap_or(0))}</span>
                    {when!(show_read_more =>
                        <span class="text-sm" style="color: var(--accent);">"read essay"</span>
                    )}
                </div>
            </div>
        </a>
    }
}

#[component]
pub fn ProjectArticle(project: &Project) -> Node {
    let brief = project
        .description
        .is_empty()
        .then(|| &project.name)
        .unwrap_or(&project.description);
    rsx! {
        <article class="group flex flex-col h-full card-item rounded-lg p-5 transition-all">
            <div class="flex flex-col gap-4 flex-1">
                <h2 class="text-xl md:text-2xl font-semibold text-zinc-100 group-hover:text-white">
                    {&project.name}
                </h2>
                <p class="text-sm md:text-base text-zinc-400 leading-relaxed">
                    {&brief[0..(brief.len().min(120))]}...
                </p>
                {if !project.tags.is_empty() {
                    rsx! {
                        <div class="flex flex-wrap gap-1.5">
                            {project.tags.iter().take(4).map(|tag| rsx! {
                                <span class="text-xs px-2 py-0.5 rounded-full border border-zinc-800 text-zinc-500">{tag.replace('-', " ")}</span>
                            })}
                        </div>
                    }
                } else {
                    rsx! { <></> }
                }}
                <div class="flex items-center justify-between flex-wrap gap-2 mt-auto">
                    <span class="flex items-center gap-3 text-xs text-zinc-500">
                        <span><i class="fas fa-star mr-1"></i>{project.stargazers_count}</span>
                        {when!(let Some(lang) = &project.language =>
                            <span><i class="fas fa-code mr-1"></i>{lang}</span>
                        )}
                    </span>
                    <a href={&project.url} class="text-sm" style="color: var(--accent);">"view project"</a>
                </div>
            </div>
        </article>
    }
}
