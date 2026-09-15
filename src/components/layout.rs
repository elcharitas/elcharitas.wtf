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
            <body class="bg-[#f7f7f4] overflow-x-hidden text-zinc-950 scheme-light">
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
        <header class="fixed top-0 left-0 right-0 z-40 nav-blur bg-[#f7f7f4]/90 border-b border-zinc-200/80">
            <div class="max-w-[1728px] mx-auto px-4 md:px-8 lg:px-20 2xl:px-16 h-11 flex items-center justify-between">
                <a href="/" class="flex items-center gap-0.5 text-sm font-bold tracking-wide text-zinc-950" aria_label="Home">
                    "elch"
                    <img src="/icon.png" alt="" class="w-4 h-4 inline-block -m-1" style="vertical-align:middle" />
                    "rit"
                    <img src="/icon.png" alt="" class="w-4 h-4 inline-block -m-1" style="vertical-align:middle" />
                    "s"
                </a>
                <div class="flex items-center gap-1">
                    <a href="/newsletter" class="icon-button" aria_label="Newsletter" title="Newsletter">
                        <i class="far fa-envelope"></i>
                    </a>
                    <a href="/connect" class="icon-button" aria_label="Contact" title="Contact">
                        <i class="far fa-message"></i>
                    </a>
                </div>
            </div>
        </header>

        <nav class="hidden lg:flex fixed left-4 xl:left-6 top-1/2 -translate-y-1/2 z-50 flex-col items-center rounded-lg border border-zinc-200 bg-white/95 p-1 shadow-sm" aria_label="Primary navigation">
            <a href="/" class="nav-icon" data_nav_href="/" aria_label="Home">
                <i class="fas fa-house"></i>
                <span class="nav-tooltip">"Home"</span>
            </a>
            <span class="nav-divider"></span>
            {NAVIGATION.iter().map(|nav| {
                let icon = match nav.href {
                    "/projects" => "fas fa-cubes",
                    "/essays" => "far fa-pen-to-square",
                    "/publications" => "fas fa-book-open",
                    "/resume" => "far fa-file-lines",
                    "/adventures" => "fas fa-route",
                    _ => "far fa-circle",
                };
                <a href={nav.href} class="nav-icon" data_nav_href={nav.href} aria_label={nav.name}>
                    <i class={icon}></i>
                    <span class="nav-tooltip">{nav.name}</span>
                </a>
            })}
            <span class="nav-divider"></span>
            <a href="/newsletter" class="nav-icon" data_nav_href="/newsletter" aria_label="Newsletter">
                <i class="far fa-envelope"></i>
                <span class="nav-tooltip">"Newsletter"</span>
            </a>
        </nav>

        <nav class="lg:hidden fixed bottom-1.5 left-2 right-2 z-50 mobile-dock" aria_label="Primary navigation">
            <a href="/" class="nav-icon" data_nav_href="/" aria_label="Home"><i class="fas fa-house"></i></a>
            {NAVIGATION.iter().map(|nav| {
                let icon = match nav.href {
                    "/projects" => "fas fa-cubes",
                    "/essays" => "far fa-pen-to-square",
                    "/publications" => "fas fa-book-open",
                    "/resume" => "far fa-file-lines",
                    "/adventures" => "fas fa-route",
                    _ => "far fa-circle",
                };
                <a href={nav.href} class="nav-icon" data_mobile_nav_href={nav.href} aria_label={nav.name}><i class={icon}></i></a>
            })}
            <a href="/newsletter" class="nav-icon" data_mobile_nav_href="/newsletter" aria_label="Newsletter"><i class="far fa-envelope"></i></a>
        </nav>
        </>
    }
}

#[component]
pub fn PageLayout(props: &LayoutProps) -> Node {
    rsx! {
        <AppLayout title={&props.title}>
            <div class="relative min-h-screen bg-[#f7f7f4]">
                <div class="relative min-h-screen flex flex-col">
                    <Navigation />

                    <main class="w-full flex-1 px-4 md:px-8 lg:px-20 2xl:px-16 pt-16 max-w-[1728px] mx-auto pb-14 lg:pb-0">
                        <section class="relative entrance-delayed space-y-4 md:space-y-6">
                            {&props.children}
                        </section>
                    </main>

                    <footer class="relative mt-8 border-t border-zinc-200">
                        <div class="max-w-[1728px] mx-auto px-4 md:px-8 lg:px-20 2xl:px-16 py-2 pb-14 lg:pb-2">
                            <div class="flex items-center justify-between gap-2">
                                <p class="text-[11px] text-zinc-500 whitespace-nowrap">
                                    <span class="hidden sm:inline">"Built with "</span>
                                    <a href="https://elcharitas.github.io/momenta" class="text-zinc-600 hover:text-zinc-950 transition-colors">
                                        "Momenta"
                                    </a>
                                    " · "
                                    <a href="https://elcharitas.wtf" class="text-zinc-600 hover:text-zinc-950 transition-colors">
                                        "K. Jonathan Irhodia"
                                    </a>
                                </p>
                                <div class="flex items-center shrink-0">
                                    <a href="https://twitter.com/iamelcharitas" class="icon-button" aria_label="X">
                                        <i class="fab fa-x-twitter"></i>
                                    </a>
                                    <a href="https://github.com/elcharitas" class="icon-button" aria_label="GitHub">
                                        <i class="fab fa-github"></i>
                                    </a>
                                    <a href="https://linkedin.com/in/elcharitas" class="icon-button" aria_label="LinkedIn">
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
