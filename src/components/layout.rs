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
    pub title: &'static str,
    pub children: Vec<Node>,
}

#[component]
pub fn AppLayout(props: &LayoutProps) -> Node {
    let page_title = METADATA.title.template.replace("%s", &props.title);
    rsx! {
        <html lang="en" class="font-plus-jakarta-sans font-calsans">
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
            </head>
            <body class="bg-black">
                {&props.children}
                <script type="module" src="https://cdn.jsdelivr.net/gh/starfederation/datastar@v1.0.0-beta.11/bundles/datastar.js"></script>
                <script src="https://cdn.jsdelivr.net/npm/driver.js@latest/dist/driver.js.iife.js">
                    const driver = window.driver.js.driver;
                    const driverObj = driver();

                    const elements = Array.from(document.querySelectorAll("[data-tour]"));
                    const steps = elements.map((element) => ({
                        element: "[data-tour=\"${element.getAttribute(\"data-tour\")}\"]",
                        popover: {
                            title: element.getAttribute("data-tour-title") ?? "",
                            description: element.getAttribute("data-tour-description") ?? "",
                            side: element.getAttribute("data-tour-position") as Side,
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
                </script>
            </body>
        </html>
    }
}

#[component]
pub fn Navigation() -> Node {
    rsx! {
        <header class="fixed top-0 left-0 right-0 z-50 bg-black/50 backdrop-blur-md border-b border-zinc-800">
            <div class="container mx-auto px-6 h-16 flex items-center justify-between">
                <a href="/" class="text-zinc-50 font-semibold text-xl hover:text-zinc-300 transition-colors">
                    "elch🔥rit🔥s"
                </a>
                <nav class="flex items-center space-x-8">
                    {NAVIGATION.iter().map(|nav| {
                        rsx! {
                            <a
                                href={nav.href}
                                class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors"
                            >
                                {nav.name}
                            </a>
                        }
                    })}
                    <a
                        href="/newsletter"
                        class="px-4 py-2 text-sm text-white  border border-yellow-700 rounded hover:bg-zinc-800 hover:scale-110 hover:rounded-xl duration-1000"
                        title="Newsletter"
                    >
                        Newsletter
                    </a>
                </nav>
            </div>
        </header>
    }
}

#[component]
pub fn PageLayout(props: &LayoutProps) -> Node {
    rsx! {
        <AppLayout title={props.title}>
            <div class="relative min-h-screen bg-gradient-to-tl from-black via-zinc-600/20 to-black">
              <div class="relative pb-16">
                <Navigation />
                <div class="px-6 pt-12 md:mx-auto space-y-8 max-w-7xl lg:px-8 md:space-y-16 md:pt-24 lg:pt-32 min-h-[85vh]">
                  {&props.children}
                </div>
              </div>
              <div class="flex justify-center text-sm text-zinc-50 py-8 border-t border-zinc-800">
                "Designed and developed by "
                <a
                  href="https://elcharitas.wtf"
                  class="font-medium text-zinc-500 hover:text-zinc-300 mx-2"
                >
                  Jonathan Irhodia
                </a>
                " ©️"
              </div>
            </div>
        </AppLayout>
    }
}
