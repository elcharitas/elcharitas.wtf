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
            class="garden-index-row group"
        >
            <span class="garden-index-meta">
                {category.map_or("general", |c| &c.name)}
                {when!(let Some(published_at) = &post.published_at =>
                    <time datetime={published_at}>{format_published_date(published_at)}</time>
                )}
            </span>
            <span class="garden-index-copy">
                <strong>{&post.title}</strong>
                <span>{&post.brief[0..(post.brief.len().min(140))]}</span>
            </span>
            {when!(show_read_more => <span class="garden-index-arrow">"→"</span>)}
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
        <a href={&project.url} class="garden-index-row group" target="_blank" rel="noopener noreferrer">
            <span class="garden-index-meta">
                {project.language.as_deref().unwrap_or("project")}
                <span>{format!("{} stars", project.stargazers_count)}</span>
            </span>
            <span class="garden-index-copy">
                <strong>{&project.name}</strong>
                <span>{&brief[0..(brief.len().min(140))]}</span>
            </span>
            <span class="garden-index-arrow">"↗"</span>
        </a>
    }
}
