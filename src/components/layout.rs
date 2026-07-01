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
            shortcut: "/icon.png".into(),
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
        <html lang="en-US" class="scroll-smooth" style="font-family: 'DM Sans', sans-serif;">
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
                <link rel="icon" href={&METADATA.icons.shortcut} />
                <link rel="shortcut icon" href={&METADATA.icons.shortcut} />
                <link rel="apple-touch-icon" href={&METADATA.icons.shortcut} />
                <link rel="stylesheet" href="/styles.css" />
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.2/css/all.min.css" />

                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="" />
                <link href="https://fonts.googleapis.com/css2?family=Raleway:wght@300;400;600;700;800&family=DM+Sans:opsz,wght@9..40,300;9..40,400;9..40,500&display=swap" rel="stylesheet" />
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/driver.js@latest/dist/driver.css"/>
            </head>
            <body class="bg-[#0a0a0a] overflow-x-hidden text-white scheme-dark">
                // Ambient background blobs
                <div class="fixed inset-0 overflow-hidden pointer-events-none">
                    <div class="absolute top-[-4rem] left-[-8rem] w-[32rem] h-[32rem] rounded-full blur-3xl" style="background: rgba(255,95,31,0.07);"></div>
                    <div class="absolute bottom-[-8rem] right-[-8rem] w-[28rem] h-[28rem] rounded-full blur-3xl" style="background: rgba(255,95,31,0.05);"></div>
                </div>

                {&props.children}

                <script type="module" src="https://cdn.jsdelivr.net/gh/starfederation/datastar@v1.0.0-beta.11/bundles/datastar.js"></script>
                <script>{r#"(function(){var p=location.pathname;['data-nav-href','data-mobile-nav-href'].forEach(function(attr){document.querySelectorAll('['+attr+']').forEach(function(a){var h=a.getAttribute(attr);if(p===h||p.startsWith(h+'/')){a.classList.add('active');}});});})();"#}</script>
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
        <header data_signals="{'mobileMenu': false}" class="fixed top-0 left-0 right-0 z-50 nav-blur bg-[#0a0a0a]/80 border-b border-zinc-800/30">
            <div class="max-w-7xl mx-auto px-4 md:px-6 py-4 flex items-center justify-between">
                <a href="/" class="flex items-center gap-1 text-xl font-bold tracking-wide text-white">
                    "elch"
                    <img src="/icon.png" alt="" class="w-6 h-6 inline-block -m-1" style="vertical-align:middle" />
                    "rit"
                    <img src="/icon.png" alt="" class="w-6 h-6 inline-block -m-1" style="vertical-align:middle" />
                    "s"
                </a>

                <div class="hidden md:flex items-center gap-5">
                    {NAVIGATION.iter().map(|nav| {
                        <a href={nav.href} class="nav-link text-sm" data_nav_href={nav.href}>{nav.name}</a>
                    })}
                    <a href="/newsletter" class="btn-accent text-sm rounded-md px-3 py-1.5">"Newsletter"</a>
                </div>

                    <button class="md:hidden p-2 social-link" data_on_click="$mobileMenu = !$mobileMenu" aria_label="Toggle Menu">
                    <i data_show="!$mobileMenu" class="fas fa-bars"></i>
                    <i data_show="$mobileMenu" class="fas fa-x"></i>
                </button>
            </div>

            <div data_show="$mobileMenu" class="menu-panel md:hidden border-t border-zinc-800 bg-black/95 px-4 pb-4">
                <div class="pt-3 space-y-2">
                    {NAVIGATION.iter().map(|nav| {
                        <a href={nav.href} class="block px-3 py-2 text-zinc-300 border border-zinc-800 rounded-lg hover:bg-zinc-900" data_mobile_nav_href={nav.href}>{nav.name}</a>
                    })}
                    <a href="/newsletter" class="block px-3 py-2 text-zinc-200 border border-zinc-700 rounded-lg hover:bg-zinc-900">"Newsletter"</a>
                </div>
            </div>
        </header>
    }
}

#[component]
pub fn PageLayout(props: &LayoutProps) -> Node {
    rsx! {
        <AppLayout title={&props.title}>
            <div class="relative min-h-screen bg-[#0a0a0a]">
                <div class="relative">
                    <Navigation />

                    <main class="px-4 md:px-6 pt-28 md:pt-32 max-w-7xl mx-auto min-h-[85vh]">
                        <section class="relative entrance-delayed space-y-10 md:space-y-12">
                            {&props.children}
                        </section>
                    </main>

                    <aside class="hidden lg:flex fixed right-8 top-1/2 -translate-y-1/2 flex-col items-center gap-4">
                        <a href="https://twitter.com/iamelcharitas" class="social-link" aria_label="Twitter">
                            <i class="fab fa-x-twitter text-base"></i>
                        </a>
                        <a href="https://github.com/elcharitas" class="social-link" aria_label="GitHub">
                            <i class="fab fa-github text-base"></i>
                        </a>
                        <a href="https://linkedin.com/in/elcharitas" class="social-link" aria_label="LinkedIn">
                            <i class="fab fa-linkedin text-base"></i>
                        </a>
                        <span class="text-[11px] tracking-[0.2em] uppercase text-zinc-600 [writing-mode:vertical-rl]">"Follow Me"</span>
                    </aside>

                    <footer class="relative mt-20 border-t border-zinc-800/50">
                        <div class="container mx-auto px-6 py-6">
                            <div class="flex flex-col sm:flex-row items-center justify-between gap-3">
                                <p class="text-xs text-zinc-500">
                                    "Built with "
                                    <a href="https://elcharitas.github.io/momenta" class="text-zinc-400 hover:text-white transition-colors">
                                        "Momenta"
                                    </a>
                                    " · "
                                    <a href="https://elcharitas.wtf" class="text-zinc-400 hover:text-white transition-colors">
                                        "Jonathan Irhodia"
                                    </a>
                                </p>
                                <div class="flex items-center gap-4">
                                    <a href="https://twitter.com/iamelcharitas" class="text-zinc-600 hover:text-zinc-300 transition-colors">
                                        <i class="fab fa-x-twitter"></i>
                                    </a>
                                    <a href="https://github.com/elcharitas" class="text-zinc-600 hover:text-white transition-colors">
                                        <i class="fab fa-github"></i>
                                    </a>
                                    <a href="https://linkedin.com/in/elcharitas" class="text-zinc-600 hover:text-zinc-300 transition-colors">
                                        <i class="fab fa-linkedin"></i>
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
