use crate::shared::NAVIGATION;
use lazy_static::lazy_static;
use momenta::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    metadata_base: String,
    title: Title,
    description: String,
    open_graph: OpenGraph,
    robots: Robots,
    twitter: Twitter,
    icons: Icons,
}

#[derive(Debug, Serialize, Deserialize)]
struct Title {
    default: String,
    template: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenGraph {
    title: String,
    description: String,
    url: String,
    site_name: String,
    images: Vec<OpenGraphImage>,
    locale: String,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenGraphImage {
    url: String,
    width: u32,
    height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Robots {
    index: bool,
    follow: bool,
    google_bot: GoogleBot,
}

#[derive(Debug, Serialize, Deserialize)]
struct GoogleBot {
    index: bool,
    follow: bool,
    #[serde(rename = "max-video-preview")]
    max_video_preview: i32,
    #[serde(rename = "max-image-preview")]
    max_image_preview: String,
    #[serde(rename = "max-snippet")]
    max_snippet: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Twitter {
    title: String,
    creator: String,
    card: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Icons {
    shortcut: String,
}

lazy_static! {
    static ref METADATA: Metadata = Metadata {
        metadata_base: String::from("https://elcharitas.wtf"),
        title: Title {
            default: "elcharitas.wtf | Software engineer with a passion for building things."
                .into(),
            template: "elcharitas.wtf | %s".into(),
        },
        description: "Software engineer with a passion for building things.".into(),
        open_graph: OpenGraph {
            title: "elcharitas.wtf | Software engineer with a passion for building things.".into(),
            description: "Software engineer with a passion for building things.".into(),
            url: String::from("https://elcharitas.wtf"),
            site_name: "elcharitas.wtf".into(),
            images: vec![OpenGraphImage {
                url: String::from("https://elcharitas.wtf/og.png"),
                width: 1920,
                height: 1080,
            }],
            locale: "en-US".into(),
            type_: "website".into(),
        },
        robots: Robots {
            index: true,
            follow: true,
            google_bot: GoogleBot {
                index: true,
                follow: true,
                max_video_preview: -1,
                max_image_preview: "large".into(),
                max_snippet: -1,
            },
        },
        twitter: Twitter {
            title: "elcharitas.wtf".into(),
            creator: "@iamelcharitas".into(),
            card: "summary_large_image".into(),
        },
        icons: Icons {
            shortcut: "/favicon.png".into(),
        },
    };
}

#[derive(Default)]
pub struct LayoutProps {
    pub title: String,
    pub children: Vec<Node>,
}

#[component]
pub fn AppLayout(props: &LayoutProps) -> Node {
    let page_title = METADATA.title.template.replace("%s", &props.title);
    rsx! {
        <html lang="en-US" class="font-plus-jakarta-sans font-calsans scroll-smooth">
            <head>
                <title>{&page_title}</title>
                <meta charset="utf-8" />
                <meta name="title" content={&page_title} />
                <meta name="description" content={&METADATA.description} />
                <meta name="viewport" content="width=device-width, initial-scale=1" />

                <meta property="og:title" content={&METADATA.open_graph.title} />
                <meta property="og:description" content={&METADATA.open_graph.description} />
                <meta property="og:url" content={METADATA.open_graph.url.as_str()} />
                <meta property="og:site_name" content={&METADATA.open_graph.site_name} />
                <meta property="og:image" content={METADATA.open_graph.images[0].url.as_str()} />
                <meta property="og:image:width" content={METADATA.open_graph.images[0].width.to_string()} />
                <meta property="og:image:height" content={METADATA.open_graph.images[0].height.to_string()} />
                <meta property="og:locale" content={&METADATA.open_graph.locale} />
                <meta property="og:type" content={&METADATA.open_graph.type_} />

                <meta name="twitter:title" content={&METADATA.twitter.title} />
                <meta name="twitter:creator" content={&METADATA.twitter.creator} />
                <meta name="twitter:card" content={&METADATA.twitter.card} />
                <meta name="twitter:image" content={METADATA.open_graph.images[0].url.as_str()} />

                <meta name="robots" content={
                    format!(
                        "{}{}",
                        if METADATA.robots.index { "index" } else { "noindex" },
                        if METADATA.robots.follow { ", follow" } else { ", nofollow" }
                    )
                } />
                <link rel="shortcut icon" href={&METADATA.icons.shortcut} />
                <link rel="stylesheet" href="/styles.css" />
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.2/css/all.min.css" />

                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="" />
                <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@200..800&display=swap" rel="stylesheet" />
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/driver.js@latest/dist/driver.css"/>

                // Enhanced custom styles
                <style>
                    {r#"
                    .glow-effect {
                        box-shadow: 0 0 20px rgba(234, 179, 8, 0.3);
                    }
                    .nav-blur {
                        backdrop-filter: blur(20px);
                        -webkit-backdrop-filter: blur(20px);
                    }
                    .gradient-border {
                        position: relative;
                        background: linear-gradient(45deg, #000, #1f2937);
                        border-radius: 12px;
                    }
                    .gradient-border::before {
                        content: '';
                        position: absolute;
                        top: 0;
                        left: 0;
                        right: 0;
                        bottom: 0;
                        border-radius: 12px;
                        padding: 1px;
                        background: linear-gradient(45deg, #eab308, #f59e0b, #d97706);
                        mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
                        mask-composite: exclude;
                        -webkit-mask-composite: xor;
                    }
                    .hover-lift {
                        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
                    }
                    .hover-lift:hover {
                        transform: translateY(-2px);
                    }
                    .text-gradient {
                        background: linear-gradient(135deg, #eab308, #f59e0b);
                        -webkit-background-clip: text;
                        -webkit-text-fill-color: transparent;
                        background-clip: text;
                    }
                    "#}
                </style>
            </head>
            <body class="bg-black overflow-x-hidden">
                // Animated background elements
                <div class="fixed inset-0 overflow-hidden pointer-events-none">
                    <div class="absolute top-1/4 left-1/4 w-96 h-96 bg-yellow-500/5 rounded-full blur-3xl animate-pulse"></div>
                    <div class="absolute bottom-1/4 right-1/4 w-96 h-96 bg-orange-500/5 rounded-full blur-3xl animate-pulse delay-1000"></div>
                    <div class="absolute top-3/4 left-1/2 w-64 h-64 bg-amber-500/5 rounded-full blur-2xl animate-pulse delay-500"></div>
                </div>

                {&props.children}

                <script type="module" src="https://cdn.jsdelivr.net/gh/starfederation/datastar@v1.0.0-beta.11/bundles/datastar.js"></script>
                <script src="https://cdn.jsdelivr.net/npm/driver.js@latest/dist/driver.js.iife.js">
                    {r#"
                    const driver = window.driver.js.driver;
                    const driverObj = driver();

                    const elements = Array.from(document.querySelectorAll("[data-tour]"));
                    const steps = elements.map((element) => ({
                        element: `[data-tour="${element.getAttribute("data-tour")}"]`,
                        popover: {
                            title: element.getAttribute("data-tour-title") ?? "",
                            description: element.getAttribute("data-tour-description") ?? "",
                            side: element.getAttribute("data-tour-position"),
                        },
                    }));
                    driverObj.setConfig({
                        steps,
                        popoverClass: "tour-wrapper",
                        onDestroyStarted: () => {
                            driverObj.destroy();
                            setIsTourEnded(true);
                        },
                    });
                    driverObj.drive();
                    "#}
                </script>
            </body>
        </html>
    }
}

#[component]
pub fn Navigation() -> Node {
    rsx! {
        <header class="fixed top-0 left-0 right-0 z-50 nav-blur bg-black/60 border-b border-zinc-800/50 transition-all duration-300">
            <div class="container mx-auto px-6 py-2 flex items-center justify-between">
                <a
                    href="/"
                    class="group flex items-center hover-lift text-white"
                >
                    "elch"
                    <span class="inline-block text-yellow-400 drop-shadow-[0_0_15px_rgba(234,179,8,0.5)]">"🔥"</span>
                    "rit"
                    <span class="inline-block delay-150 text-orange-400 drop-shadow-[0_0_15px_rgba(251,146,60,0.5)]">"🔥"</span>
                    "s"
                </a>

                <nav class="hidden md:flex items-center space-x-1">
                    {NAVIGATION.iter().map(|nav| {
                        rsx! {
                            <a
                                href={nav.href}
                                class="px-4 py-2 text-sm font-medium text-zinc-400 hover:text-white hover:bg-zinc-800/50 rounded-lg transition-all duration-200 hover-lift relative group"
                            >
                                <span class="relative z-10">{nav.name}</span>
                                <div class="absolute inset-0 bg-gradient-to-r from-yellow-500/10 to-orange-500/10 rounded-lg opacity-0 group-hover:opacity-100 transition-opacity duration-200"></div>
                            </a>
                        }
                    })}

                    <div class="ml-4">
                        <a
                            href="/newsletter"
                            class="gradient-border px-6 py-2.5 text-sm font-semibold text-white hover:text-yellow-300 transition-all duration-300 hover-lift inline-flex items-center space-x-2 group"
                            title="Newsletter"
                        >
                            <i class="fas fa-envelope text-xs"></i>
                            <span>"SUBSCRIBE"</span>
                        </a>
                    </div>
                </nav>

                <button class="md:hidden p-2 text-zinc-400 hover:text-white transition-colors">
                    <i class="fas fa-bars text-lg"></i>
                </button>
            </div>
        </header>
    }
}

#[component]
pub fn PageLayout(props: &LayoutProps) -> Node {
    rsx! {
        <AppLayout title={&props.title}>
            <div class="relative min-h-screen bg-gradient-to-br from-black via-zinc-900/50 to-black">
                // Enhanced gradient overlay
                <div class="absolute inset-0 bg-gradient-to-t from-black via-transparent to-black/50 pointer-events-none"></div>

                <div class="relative">
                    <Navigation />

                    // Main content area with improved spacing and styling
                    <main class="px-6 pt-20 md:mx-auto space-y-12 max-w-7xl lg:px-8 md:space-y-20 md:pt-32 lg:pt-40 min-h-[85vh]">
                        <div class="relative">
                            {&props.children}
                        </div>
                    </main>

                    // Enhanced footer with better styling
                    <footer class="relative mt-20 border-t border-zinc-800/50 bg-zinc-900/20 backdrop-blur-sm">
                        <div class="container mx-auto px-6 py-12">
                            <div class="flex flex-col md:flex-row items-center justify-between space-y-4 md:space-y-0">
                                // Footer content
                                <div class="flex items-center text-sm text-zinc-400">
                                    <span>"Built on "</span>
                                    <a
                                        href="https://ngyn.rs"
                                        class="font-semibold text-orange-400 hover:text-orange-300 mx-1 transition-colors duration-200 hover-lift"
                                    >
                                        "Ngyn"
                                    </a>
                                    "and "
                                    <a
                                        href="https://elcharitas.github.io/momenta"
                                        class="font-semibold text-blue-400 hover:text-blue-300 mx-1 transition-colors duration-200 hover-lift"
                                    >
                                        "Momenta"
                                    </a>
                                    <span>" by "</span>
                                    <a
                                        href="https://elcharitas.wtf"
                                        class="font-semibold text-yellow-400 hover:text-yellow-300 mx-1 transition-colors duration-200 hover-lift"
                                    >
                                        "Jonathan Irhodia"
                                    </a>
                                </div>

                                // Social links
                                <div class="flex items-center space-x-4">
                                    <a href="https://twitter.com/iamelcharitas" class="text-zinc-400 hover:text-blue-400 transition-colors duration-200 hover-lift">
                                        <i class="fab fa-x-twitter text-lg"></i>
                                    </a>
                                    <a href="https://github.com/elcharitas" class="text-zinc-400 hover:text-white transition-colors duration-200 hover-lift">
                                        <i class="fab fa-github text-lg"></i>
                                    </a>
                                    <a href="https://linkedin.com/in/elcharitas" class="text-zinc-400 hover:text-blue-500 transition-colors duration-200 hover-lift">
                                        <i class="fab fa-linkedin text-lg"></i>
                                    </a>
                                </div>
                            </div>
                        </div>
                    </footer>
                </div>
            </div>
        </AppLayout>
    }
}
