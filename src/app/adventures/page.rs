use crate::{components::PageLayout, shared::PageParams};
use momenta::prelude::*;

#[component]
pub fn AdventuresPage(_: &PageParams) -> Node {
    // Sample adventure data for demonstration
    let adventures = vec![
        (
            "Coming Soon",
            "2024",
            "🌟 New",
            "Worldwide",
            "Stay tuned for exciting adventures and stories from around the globe.",
            "/images/adventures/placeholder.svg",
            "bg-gradient-to-br from-blue-500/20 to-cyan-500/20",
            "border-blue-500/30",
        ),
        (
            "Digital Nomad Journey",
            "2024",
            "🚀 Ongoing",
            "Remote",
            "Exploring the world while building amazing software products.",
            "/images/adventures/nomad.svg",
            "bg-gradient-to-br from-purple-500/20 to-pink-500/20",
            "border-purple-500/30",
        ),
        (
            "Tech Conference Tours",
            "2024",
            "📅 Planned",
            "Global",
            "Speaking and attending conferences across different continents.",
            "/images/adventures/conferences.svg",
            "bg-gradient-to-br from-green-500/20 to-emerald-500/20",
            "border-green-500/30",
        ),
        (
            "Open Source Expeditions",
            "2024",
            "💻 Active",
            "Virtual",
            "Contributing to projects and building communities worldwide.",
            "/images/adventures/opensource.svg",
            "bg-gradient-to-br from-orange-500/20 to-red-500/20",
            "border-orange-500/30",
        ),
        (
            "Cultural Immersion",
            "2025",
            "🎯 Upcoming",
            "Multiple",
            "Learning from different cultures and their approaches to technology.",
            "/images/adventures/culture.svg",
            "bg-gradient-to-br from-yellow-500/20 to-amber-500/20",
            "border-yellow-500/30",
        ),
        (
            "Adventure Photography",
            "2024",
            "📸 Ongoing",
            "Various",
            "Capturing moments and stories through the lens of technology.",
            "/images/adventures/photography.svg",
            "bg-gradient-to-br from-indigo-500/20 to-purple-500/20",
            "border-indigo-500/30",
        ),
    ];

    rsx! {
        <PageLayout title="Adventures">
            // Enhanced hero section with animated elements
            <div class="relative mb-16 md:mb-24">
                // Animated background elements
                <div class="absolute inset-0 overflow-hidden">
                    <div class="absolute top-10 left-10 w-32 h-32 bg-blue-500/10 rounded-full blur-2xl animate-pulse"></div>
                    <div class="absolute bottom-10 right-10 w-40 h-40 bg-purple-500/10 rounded-full blur-2xl animate-pulse delay-1000"></div>
                    <div class="absolute top-1/2 left-1/2 w-24 h-24 bg-yellow-500/10 rounded-full blur-xl animate-pulse delay-500"></div>
                </div>

                <div class="relative text-center">
                    // Enhanced title with better effects
                    <div class="relative inline-block">
                        <h1 class="text-4xl md:text-6xl lg:text-7xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-yellow-400 via-orange-400 to-red-400 animate-title mb-6">
                            "Adventures "
                            <span class="inline-block animate-bounce text-blue-400 drop-shadow-[0_0_15px_rgba(59,130,246,0.5)]">"🌎"</span>
                        </h1>

                        // Glowing backdrop
                        <div class="absolute inset-0 blur-3xl bg-gradient-to-r from-yellow-400/20 via-orange-400/20 to-red-400/20 scale-150 -z-10"></div>
                    </div>

                    // Enhanced description with better typography
                    <div class="max-w-4xl mx-auto">
                        <p class="text-lg md:text-xl text-zinc-300 leading-relaxed mb-8 animate-fade-in">
                            "Join me on my journey around the world, exploring new places, cultures, and experiences."
                            <br class="hidden md:block" />
                            "From digital nomadism to tech conferences, every adventure tells a story."
                        </p>

                        // Stats section
                        <div class="flex flex-wrap justify-center gap-8 md:gap-12 text-center animate-fade-in delay-300">
                            <div class="group">
                                <div class="text-2xl md:text-3xl font-bold text-yellow-400 group-hover:text-yellow-300 transition-colors">
                                    "15+"
                                </div>
                                <div class="text-sm text-zinc-400 group-hover:text-zinc-300 transition-colors">
                                    "Countries Visited"
                                </div>
                            </div>
                            <div class="group">
                                <div class="text-2xl md:text-3xl font-bold text-blue-400 group-hover:text-blue-300 transition-colors">
                                    "50+"
                                </div>
                                <div class="text-sm text-zinc-400 group-hover:text-zinc-300 transition-colors">
                                    "Cities Explored"
                                </div>
                            </div>
                            <div class="group">
                                <div class="text-2xl md:text-3xl font-bold text-purple-400 group-hover:text-purple-300 transition-colors">
                                    "100+"
                                </div>
                                <div class="text-sm text-zinc-400 group-hover:text-zinc-300 transition-colors">
                                    "Stories Shared"
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Filter/Category section
            <div class="mb-8 md:mb-12 animate-fade-in delay-500">
                <div class="flex flex-wrap justify-center gap-2 md:gap-4">
                    <button class="px-4 py-2 text-sm font-medium text-white bg-gradient-to-r from-yellow-500 to-orange-500 rounded-full hover:from-yellow-400 hover:to-orange-400 transition-all duration-300 hover:scale-105">
                        "All Adventures"
                    </button>
                    <button class="px-4 py-2 text-sm font-medium text-zinc-400 hover:text-white border border-zinc-700 hover:border-zinc-600 rounded-full hover:bg-zinc-800/50 transition-all duration-300">
                        "Travel"
                    </button>
                    <button class="px-4 py-2 text-sm font-medium text-zinc-400 hover:text-white border border-zinc-700 hover:border-zinc-600 rounded-full hover:bg-zinc-800/50 transition-all duration-300">
                        "Tech Events"
                    </button>
                    <button class="px-4 py-2 text-sm font-medium text-zinc-400 hover:text-white border border-zinc-700 hover:border-zinc-600 rounded-full hover:bg-zinc-800/50 transition-all duration-300">
                        "Culture"
                    </button>
                </div>
            </div>

            // Enhanced adventures grid
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 md:gap-8 animate-fade-in delay-700">
                {adventures.iter().enumerate().map(|(index, (title, year, status, location, description, image, gradient, border))| {
                    rsx! {
                        <article
                            class="group relative flex flex-col overflow-hidden rounded-xl border border-zinc-800 bg-zinc-900/80 backdrop-blur-sm hover:border-zinc-700 transition-all duration-500 hover:scale-[1.02] hover:shadow-2xl hover:shadow-black/50"
                            style={format!("animation-delay: {}ms", index * 100)}
                        >
                            // Animated gradient overlay
                            <div class={format!("absolute inset-0 {} opacity-0 group-hover:opacity-100 transition-opacity duration-500", gradient)}></div>

                            // Image section with enhanced effects
                            <div class="relative aspect-video overflow-hidden">
                                // Placeholder for actual image
                                <div class="absolute inset-0 bg-gradient-to-br from-zinc-800 to-zinc-900 flex items-center justify-center">
                                    <div class="text-6xl opacity-20">
                                        {match title {
                                            title if title.contains("Nomad") => "💻",
                                            title if title.contains("Conference") => "🎤",
                                            title if title.contains("Source") => "🚀",
                                            title if title.contains("Cultural") => "🌏",
                                            title if title.contains("Photography") => "📸",
                                            _ => "🌟"
                                        }}
                                    </div>
                                </div>

                                // Enhanced overlays
                                <div class="absolute inset-0 bg-gradient-to-t from-zinc-900/90 via-zinc-900/20 to-transparent"></div>
                                <div class="absolute inset-0 bg-gradient-to-br from-transparent via-transparent to-black/30 group-hover:to-black/50 transition-all duration-500"></div>

                                // Status badge
                                <div class="absolute top-4 left-4 z-10">
                                    <span class={format!("inline-flex items-center px-3 py-1 rounded-full text-xs font-medium {} border text-white", border)}>
                                        {status}
                                    </span>
                                </div>

                                // Interactive overlay button
                                <div class="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all duration-300">
                                    <button class="px-4 py-2 bg-white/20 backdrop-blur-md rounded-lg text-white font-medium hover:bg-white/30 transition-all duration-200">
                                        "Explore"
                                    </button>
                                </div>
                            </div>

                            // Enhanced content section
                            <div class="relative flex flex-1 flex-col justify-between p-6">
                                // Header with metadata
                                <div class="flex-1">
                                    <div class="flex items-center justify-between mb-3">
                                        <time class="text-sm font-medium text-zinc-400 group-hover:text-zinc-300 transition-colors">
                                            {year}
                                        </time>
                                        <div class="flex items-center gap-2 text-xs text-zinc-500">
                                            <i class="fas fa-map-marker-alt"></i>
                                            <span>{location}</span>
                                        </div>
                                    </div>

                                    // Title and description
                                    <h2 class="text-xl font-bold text-zinc-100 group-hover:text-white transition-all duration-300 mb-3 group-hover:translate-x-1">
                                        {title}
                                    </h2>
                                    <p class="text-sm text-zinc-400 group-hover:text-zinc-300 leading-relaxed transition-colors duration-300">
                                        {description}
                                    </p>
                                </div>

                                // Enhanced footer
                                <div class="mt-6 pt-4 border-t border-zinc-800 group-hover:border-zinc-700 transition-colors duration-300">
                                    <div class="flex items-center justify-between">
                                        <div class="flex items-center gap-4 text-xs text-zinc-500">
                                            <span class="flex items-center gap-1">
                                                <i class="far fa-calendar"></i>
                                                "Updated recently"
                                            </span>
                                        </div>

                                        // Action buttons
                                        <div class="flex items-center gap-2">
                                            <button class="p-2 rounded-lg text-zinc-400 hover:text-white hover:bg-zinc-800 transition-all duration-200">
                                                <i class="far fa-heart text-sm"></i>
                                            </button>
                                            <button class="p-2 rounded-lg text-zinc-400 hover:text-white hover:bg-zinc-800 transition-all duration-200">
                                                <i class="fas fa-share-alt text-sm"></i>
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            // Subtle glow effect
                            <div class="absolute inset-0 rounded-xl opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none" style="box-shadow: 0 0 30px rgba(234, 179, 8, 0.1);"></div>
                        </article>
                    }
                })}
            </div>

            // Call-to-action section
            <div class="mt-16 md:mt-24 text-center animate-fade-in delay-1000">
                <div class="max-w-2xl mx-auto p-8 rounded-2xl bg-gradient-to-br from-zinc-900/80 to-zinc-800/80 backdrop-blur-sm border border-zinc-700">
                    <h3 class="text-2xl md:text-3xl font-bold text-white mb-4">
                        "Want to join the adventure?"
                    </h3>
                    <p class="text-zinc-300 mb-6 leading-relaxed">
                        "Follow along as I document my journeys, share insights, and connect with amazing people around the world."
                    </p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center">
                        <a
                            href="/newsletter"
                            class="inline-flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-yellow-500 to-orange-500 text-black font-semibold rounded-lg hover:from-yellow-400 hover:to-orange-400 transition-all duration-300 hover:scale-105"
                        >
                            <i class="fas fa-envelope"></i>
                            "Get Updates"
                        </a>
                        <a
                            href="/connect"
                            class="inline-flex items-center gap-2 px-6 py-3 border-2 border-zinc-600 text-white font-semibold rounded-lg hover:border-zinc-500 hover:bg-zinc-800/50 transition-all duration-300"
                        >
                            <i class="fas fa-comment"></i>
                            "Share Your Story"
                        </a>
                    </div>
                </div>
            </div>

            // Custom animations
            <style>
                {r#"
                @keyframes fade-in {
                    from { opacity: 0; transform: translateY(30px); }
                    to { opacity: 1; transform: translateY(0); }
                }
                
                @keyframes title {
                    from { opacity: 0; transform: scale(0.8) translateY(-20px); }
                    to { opacity: 1; transform: scale(1) translateY(0); }
                }
                
                .animate-fade-in {
                    animation: fade-in 0.8s ease-out forwards;
                }
                
                .animate-title {
                    animation: title 1.2s cubic-bezier(0.4, 0, 0.2, 1) forwards;
                }
                
                .delay-300 { animation-delay: 300ms; }
                .delay-500 { animation-delay: 500ms; }
                .delay-700 { animation-delay: 700ms; }
                .delay-1000 { animation-delay: 1000ms; }
                "#}
            </style>
        </PageLayout>
    }
}
