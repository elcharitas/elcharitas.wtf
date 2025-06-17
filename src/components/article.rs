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
            href={format!("/writings/{}", post.slug)}
            class="block group relative overflow-hidden flex flex-col relative bg-zinc-900 rounded-lg h-full"
        >
            {when!(let Some(cover_url) = &post.cover_image =>
                <div class="relative overflow-hidden">
                    <img
                        src={&cover_url.url}
                        alt={format!("Cover image for {}", post.title)}
                        class="w-full h-32 md:h-48 object-cover group-hover:scale-105 transition-transform duration-300"
                    />
                    <div class="absolute inset-0 bg-gradient-to-t from-zinc-900/60 to-transparent opacity-60"></div>
                </div>
            )}

            <div class="relative p-6 flex flex-col justify-between">
                <div class="flex justify-between items-start mb-4 gap-4">
                    <div class="flex flex-col space-y-2">
                        {when!(let Some(published_at) = &post.published_at =>
                            <time
                                datetime={&*published_at}
                                class="text-xs font-medium text-yellow-400/80 group-hover:text-yellow-300 transition-colors duration-300 flex items-center space-x-1"
                            >
                                <i class="far fa-calendar-alt text-xs"></i>
                                <span>
                                    {chrono::DateTime::parse_from_rfc3339(&published_at).unwrap_or_default().format("%B %d, %Y").to_string()}
                                </span>
                            </time>
                        )}

                        <span class="inline-flex items-center px-2.5 py-1 text-xs font-medium bg-zinc-800/60 text-zinc-300 rounded-full border border-zinc-700/50 group-hover:border-yellow-500/30 group-hover:bg-yellow-500/10 transition-all duration-300 lowercase">
                            {category.map_or("general", |c| &c.name)}
                        </span>
                    </div>

                    <div class="flex flex-col items-end space-y-2 text-xs text-zinc-400 group-hover:text-zinc-300 transition-colors duration-300">
                        <div class="flex items-center space-x-1">
                            <i class="far fa-comment text-yellow-400/60"></i>
                            <span class="font-medium">{post.comments.total_documents}</span>
                        </div>
                        <div class="flex items-center space-x-1">
                            <i class="far fa-eye text-orange-400/60"></i>
                            <span class="font-medium">{post.views}</span>
                        </div>
                    </div>
                </div>

                <h2 class="text-lg md:text-xl font-bold text-zinc-50 group-hover:text-white transition-colors duration-300 mb-1">
                    {&post.title}
                </h2>

                <p class="text-sm md:text-base text-zinc-400 group-hover:text-zinc-200 leading-relaxed flex-grow transition-colors duration-300 line-clamp-3">
                    {&post.brief[0..(post.brief.len().min(100))]}...
                </p>

                {when!(show_read_more =>
                    <div class="flex-1 mt-2">
                        <div class="flex items-center justify-between">
                            <div class="flex items-center space-x-2 text-yellow-400 group-hover:text-yellow-300 font-medium text-sm transition-all duration-300 group-hover:transform group-hover:translate-x-1">
                                <span>"Read article"</span>
                                <i class="fas fa-arrow-right text-xs group-hover:translate-x-1 transition-transform duration-300"></i>
                            </div>

                            <span class="text-xs text-zinc-500 group-hover:text-zinc-400 transition-colors duration-300">
                                "5 min read"
                            </span>
                        </div>

                        <div class="mt-3 h-0.5 bg-zinc-800 rounded-full overflow-hidden">
                            <div class="h-full bg-gradient-to-r from-yellow-400 to-orange-400 transform -translate-x-full group-hover:translate-x-0 transition-transform duration-700 ease-out"></div>
                        </div>
                    </div>
                )}
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
        <article class="flex group relative">
            <div class="absolute -inset-1 bg-gradient-to-r from-yellow-400 via-orange-500 to-yellow-400 rounded-xl opacity-0 group-hover:opacity-100 transition-opacity duration-500 -z-10">
                <div class="absolute inset-0 bg-gradient-to-r from-yellow-400 via-orange-500 via-red-500 via-orange-500 to-yellow-400 rounded-xl animate-spin-slow opacity-0 group-hover:opacity-100 transition-opacity duration-500" style="animation-duration: 3s;"></div>
            </div>

            <div class="flex-1 flex flex-col relative p-6 md:p-8 bg-gradient-to-br from-zinc-900/80 to-zinc-800/50 backdrop-blur-sm rounded-xl overflow-hidden border border-zinc-700/50 group-hover:border-transparent transition-all duration-500 hover:transform hover:scale-[1.02] hover:shadow-2xl hover:shadow-yellow-500/10">
                <div class="flex items-start justify-between mb-4">
                    <div class="flex items-center space-x-3">
                        <div>
                            <h2 class="text-xl md:text-2xl font-bold text-zinc-50 group-hover:text-white transition-colors duration-300 mb-1 truncate">
                                {&project.name}
                            </h2>
                        </div>
                    </div>

                    <div class="flex flex-col items-end space-y-1 text-xs text-zinc-400">
                        <span class="inline-flex items-center px-2 py-1 text-xs font-medium bg-green-500/20 text-green-400 rounded-full border border-green-500/30">
                            <span class="mr-1 animate-pulse"><i class="fas fa-star text-yellow-400"></i></span>
                            <span>{project.stargazers_count}</span>
                        </span>
                    </div>
                </div>

                <p class="text-zinc-300 group-hover:text-zinc-100 leading-relaxed mb-6 transition-colors duration-300">
                    {&brief[0..(brief.len().min(60))]}...
                </p>

                <div class="flex-1 mb-6">
                    <div class="flex flex-wrap gap-2">
                        {project.tags.iter().map(|tag| {
                            rsx! {
                                <span class="inline-flex items-center px-3 py-1.5 text-xs font-medium bg-zinc-800/60 text-zinc-300 rounded-full border border-zinc-600/50 group-hover:border-yellow-500/30 group-hover:bg-yellow-500/10 group-hover:text-yellow-300 transition-all duration-300 hover:scale-105">
                                    {tag}
                                </span>
                            }
                        })}
                    </div>
                </div>

                <div class="flex items-center justify-between pt-4 border-t border-zinc-700/50 group-hover:border-zinc-600/50 transition-colors duration-300">
                    <div class="flex items-center space-x-4">
                        <a
                            href={&project.url}
                            class="inline-flex items-center space-x-2 px-4 py-2 bg-zinc-800/60 text-zinc-300 text-sm font-medium rounded-lg border border-zinc-600/50 hover:border-zinc-500 hover:bg-zinc-700/60 hover:text-white transition-all duration-300 hover:scale-105"
                        >
                            <i class="fab fa-github text-sm"></i>
                        </a>
                    </div>

                    <span class="text-xs text-zinc-500 group-hover:text-zinc-400 transition-colors duration-300">
                        {chrono::DateTime::parse_from_rfc3339(&project.updated_at).unwrap_or_default().format("%b %d, %Y").to_string()}
                    </span>
                </div>

                <div class="mt-4">
                    <div class="flex items-center justify-between text-xs text-zinc-400 mb-2">
                        <span>"Watching"</span>
                        <span>{project.watching}</span>
                    </div>
                    <div class="h-1.5 bg-zinc-800 rounded-full overflow-hidden">
                        <div class="h-full bg-gradient-to-r from-yellow-400 to-orange-500 rounded-full transition-all duration-1000 ease-out" style={format!("width: {}%", project.watching)}></div>
                    </div>
                </div>
            </div>
        </article>
    }
}
