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
            default: "Kehinde Jonathan Irhodia | Software engineer and researcher".into(),
            template: "%s | Kehinde Jonathan Irhodia".into(),
        },
        description: "Software engineer and biodiagnostics researcher working across systems, AI, and practical diagnostics.".into(),
        open_graph: OpenGraph {
            title: "Kehinde Jonathan Irhodia | Software engineer and researcher".into(),
            description: "Software engineer and biodiagnostics researcher working across systems, AI, and practical diagnostics.".into(),
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
            title: "Kehinde Jonathan Irhodia".into(),
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
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="" />
                <link href="https://fonts.googleapis.com/css2?family=Manrope:wght@400;500;600;700&family=DM+Sans:opsz,wght@9..40,300;9..40,400;9..40,500&display=swap" rel="stylesheet" />
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/driver.js@latest/dist/driver.css"/>
            </head>
            <body class="bg-[#f8f7f3] overflow-x-hidden text-zinc-950 scheme-light">
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
        <>
        <header class="sticky top-0 z-40 nav-blur bg-[#f8f7f3]/92 border-b border-zinc-900/10">
            <div class="max-w-[1440px] mx-auto px-5 md:px-8 lg:px-12 h-14 flex items-center justify-between gap-6">
                <a href="/" class="flex items-baseline gap-2 text-zinc-950" aria_label="Home">
                    <span class="text-base font-semibold tracking-[-0.04em]">"KJI."</span>
                    <span class="hidden sm:inline text-xs text-zinc-500">"Kehinde Jonathan Irhodia"</span>
                </a>

                <div class="flex items-center gap-5">
                    <nav class="hidden lg:flex items-center gap-5" aria_label="Primary navigation">
                        <a href="/" class="site-nav-link" data_nav_href="/">"Home"</a>
                        {NAVIGATION.iter().map(|nav| {
                            <a href={nav.href} class="site-nav-link" data_nav_href={nav.href}>{nav.name}</a>
                        })}
                        <a href="/newsletter" class="site-nav-link" data_nav_href="/newsletter">"Newsletter"</a>
                    </nav>
                    <a href="/connect" class="contact-link">"Let's talk ↗"</a>
                    <div class="mobile-menu lg:hidden">
                        <button type="button">"Menu"</button>
                        <nav class="mobile-menu-panel" aria_label="Mobile navigation">
                            <a href="/" data_mobile_nav_href="/">"Home"</a>
                            {NAVIGATION.iter().map(|nav| {
                                <a href={nav.href} data_mobile_nav_href={nav.href}>{nav.name}</a>
                            })}
                            <a href="/newsletter" data_mobile_nav_href="/newsletter">"Newsletter"</a>
                        </nav>
                    </div>
                </div>
            </div>
        </header>
        </>
    }
}

#[component]
pub fn PageLayout(props: &LayoutProps) -> Node {
    rsx! {
        <AppLayout title={&props.title}>
            <div class="relative min-h-screen bg-[#f8f7f3]">
                <div class="relative min-h-screen flex flex-col">
                    <Navigation />

                    <main class="w-full flex-1 px-5 md:px-8 lg:px-12 max-w-[1440px] mx-auto">
                        <section class="relative entrance-delayed">
                            {&props.children}
                        </section>
                    </main>

                    <footer class="relative border-t border-zinc-900/10">
                        <div class="max-w-[1440px] mx-auto px-5 md:px-8 lg:px-12 py-5">
                            <div class="flex items-center justify-between gap-4 text-xs text-zinc-500">
                                <p class="whitespace-nowrap">"K. Jonathan Irhodia"</p>
                                <div class="flex items-center gap-4">
                                    <a href="https://github.com/elcharitas" class="footer-link">"GitHub"</a>
                                    <a href="https://linkedin.com/in/elcharitas" class="footer-link">"LinkedIn"</a>
                                    <a href="https://twitter.com/iamelcharitas" class="footer-link">"X"</a>
                                </div>
                            </div>
                        </div>
                    </footer>
                </div>
            </div>
        </AppLayout>
    }
}
