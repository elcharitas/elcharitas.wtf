use crate::components::AppLayout;
use crate::shared::{PageParams, NAVIGATION};
use momenta::prelude::*;

#[component]
pub fn HomePage(_: &PageParams) -> Node {
    rsx! {
        <AppLayout title="Home">
            <div class="relative flex flex-col items-center justify-center w-screen h-screen overflow-hidden bg-gradient-to-br from-black via-zinc-900/50 to-black">
                // Animated background elements
                <div class="absolute inset-0 overflow-hidden pointer-events-none">
                    // Floating orbs
                    <div class="absolute top-1/4 left-1/4 w-96 h-96 bg-yellow-500/10 rounded-full blur-3xl animate-pulse"></div>
                    <div class="absolute bottom-1/4 right-1/4 w-96 h-96 bg-orange-500/10 rounded-full blur-3xl animate-pulse delay-1000"></div>
                    <div class="absolute top-3/4 left-1/2 w-64 h-64 bg-amber-500/10 rounded-full blur-2xl animate-pulse delay-500"></div>
                </div>

                // Enhanced navigation with better animations
                <nav class="relative z-10 my-8 md:my-16 animate-fade-in">
                    <div class="flex items-center justify-center gap-1 md:gap-4 px-6 py-3 rounded-full backdrop-blur-md bg-zinc-900/30 border border-zinc-800/50">
                        {NAVIGATION.iter().enumerate().map(|(i, item)| rsx! {
                            <a
                                href={item.href}
                                class="relative px-3 md:px-4 py-2 text-xs md:text-sm font-medium text-zinc-400 hover:text-white transition-all duration-500 rounded-full hover:bg-zinc-800/50 group"
                                style={format!("animation-delay: {}ms", i * 100)}
                            >
                                <span class="relative z-10">{item.name}</span>
                                // Hover effect background
                                <div class="absolute inset-0 bg-gradient-to-r from-yellow-500/20 to-orange-500/20 rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                            </a>
                        })}
                    </div>
                </nav>

                // Enhanced main title with better effects
                <div class="relative z-10 text-center mb-8">
                    // Glowing backdrop for title
                    <div class="absolute inset-0 blur-3xl bg-gradient-to-r from-yellow-400/20 via-orange-400/20 to-amber-400/20 rounded-full scale-150"></div>

                    <h1 class="relative text-4xl sm:text-6xl md:text-8xl lg:text-9xl font-bold text-white duration-1000 cursor-default font-display animate-title">
                        "elch"
                        <span class="inline-block text-yellow-400 drop-shadow-[0_0_15px_rgba(234,179,8,0.5)]">"🔥"</span>
                        "rit"
                        <span class="inline-block delay-150 text-orange-400 drop-shadow-[0_0_15px_rgba(251,146,60,0.5)]">"🔥"</span>
                        "s"
                    </h1>

                    // Subtitle with typing effect
                    <div class="mt-4 text-lg md:text-xl text-zinc-400 font-light tracking-wide">
                        <span class="text-yellow-400">"{"</span>
                        <span class="text-zinc-300">"Software Engineer"</span>
                        <span class="text-yellow-400">"}"</span>
                    </div>
                </div>

                // Enhanced description section
                <div class="relative z-10 max-w-4xl mx-4 text-center animate-fade-in">
                    <div class="backdrop-blur-md bg-zinc-900/20 rounded-2xl border border-zinc-800/30 p-6 md:p-8">
                        <p class="text-sm md:text-base text-zinc-300 leading-relaxed mb-6">
                            "Hi, I'm "
                            <span class="font-semibold text-white">"Elcharitas"</span>
                            "."
                        </p>
                    </div>
                </div>

                // Enhanced social links section
                <div class="relative z-10 mt-8 md:mt-12 animate-fade-in">
                    <div class="flex items-center justify-center gap-2 mb-4">
                        <div class="h-px w-8 bg-gradient-to-r from-transparent to-zinc-600"></div>
                        <span class="text-xs text-zinc-500 font-medium">"CONNECT"</span>
                        <div class="h-px w-8 bg-gradient-to-l from-transparent to-zinc-600"></div>
                    </div>

                    <div class="flex justify-center items-center gap-4">
                        // GitHub
                        <a
                            href="https://github.com/elcharitas"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="group relative p-3 rounded-full bg-zinc-800/50 border border-zinc-700 hover:border-zinc-600 hover:bg-zinc-700/50 transition-all duration-300 hover:scale-110 hover:shadow-lg hover:shadow-zinc-400/20"
                            title="I collaborate on github often"
                        >
                            <i class="fab fa-github text-xl text-zinc-400 group-hover:text-white transition-colors duration-300"></i>
                            <div class="absolute inset-0 bg-gradient-to-r from-zinc-500/20 to-zinc-400/20 rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                        </a>

                        // LinkedIn
                        <a
                            href="https://linkedin.com/in/elcharitas"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="group relative p-3 rounded-full bg-zinc-800/50 border border-zinc-700 hover:border-blue-500/50 hover:bg-blue-900/20 transition-all duration-300 hover:scale-110 hover:shadow-lg hover:shadow-blue-400/20"
                            title="Connect with me on linkedin"
                        >
                            <i class="fab fa-linkedin text-xl text-zinc-400 group-hover:text-blue-400 transition-colors duration-300"></i>
                            <div class="absolute inset-0 bg-gradient-to-r from-blue-500/20 to-blue-400/20 rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                        </a>

                        // Instagram
                        <a
                            href="https://instagram.com/iamelcharitas"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="group relative p-3 rounded-full bg-zinc-800/50 border border-zinc-700 hover:border-pink-500/50 hover:bg-pink-900/20 transition-all duration-300 hover:scale-110 hover:shadow-lg hover:shadow-pink-400/20"
                            title="Connect with me on instagram"
                        >
                            <i class="fab fa-instagram text-xl text-zinc-400 group-hover:text-pink-400 transition-colors duration-300"></i>
                            <div class="absolute inset-0 bg-gradient-to-r from-pink-500/20 via-purple-500/20 to-yellow-500/20 rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                        </a>

                        // Twitter/X
                        <a
                            href="https://twitter.com/iamelcharitas"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="group relative p-3 rounded-full bg-zinc-800/50 border border-zinc-700 hover:border-sky-500/50 hover:bg-sky-900/20 transition-all duration-300 hover:scale-110 hover:shadow-lg hover:shadow-sky-400/20"
                            title="Follow me on twitter"
                        >
                            <i class="fab fa-twitter text-xl text-zinc-400 group-hover:text-sky-400 transition-colors duration-300"></i>
                            <div class="absolute inset-0 bg-gradient-to-r from-sky-500/20 to-blue-400/20 rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                        </a>
                    </div>
                </div>

                // Floating particles effect
                <div class="absolute inset-0 pointer-events-none overflow-hidden">
                    {(0..20).map(|i| rsx! {
                        <div
                            class="absolute w-1 h-1 bg-yellow-400/30 rounded-full animate-pulse"
                            style={format!("
                                left: {}%; 
                                top: {}%; 
                                animation-delay: {}s; 
                                animation-duration: {}s;
                            ", 
                                (i * 17) % 100,
                                (i * 23) % 100,
                                i as f32 * 0.5,
                                3.0 + (i as f32 * 0.2)
                            )}
                        ></div>
                    })}
                </div>
            </div>

            // Custom CSS for enhanced animations
            <style>
                {r#"
                @keyframes fade-in {
                    from { opacity: 0; transform: translateY(20px); }
                    to { opacity: 1; transform: translateY(0); }
                }
                
                @keyframes title {
                    from { opacity: 0; transform: scale(0.5) rotate(-10deg); }
                    to { opacity: 1; transform: scale(1) rotate(0deg); }
                }
                
                .animate-fade-in {
                    animation: fade-in 1s ease-out forwards;
                }
                
                .animate-title {
                    animation: title 1.5s cubic-bezier(0.4, 0, 0.2, 1) forwards;
                }
                
                .text-edge-outline {
                    text-shadow: 
                        0 0 10px rgba(234, 179, 8, 0.3),
                        0 0 20px rgba(234, 179, 8, 0.2),
                        0 0 30px rgba(234, 179, 8, 0.1);
                }
                "#}
            </style>
        </AppLayout>
    }
}
