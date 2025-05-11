use crate::shared::*;
use crate::{derive_component, derive_props};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use simple_rsx::{rsx, Node};

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
            template: "%s | elcharitas.wtf".into(),
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

pub struct LayoutProps {
    pub title: String,
    pub children: Node,
}

#[allow(non_snake_case)]
pub fn AppLayout(props: LayoutProps) -> Node {
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

                <meta name="robots" content={
                    format!(
                        "{}{}",
                        if METADATA.robots.index { "index" } else { "noindex" },
                        if METADATA.robots.follow { ", follow" } else { ", nofollow" }
                    )
                } />
                <link rel="shortcut icon" href={&METADATA.icons.shortcut} />
                <link rel="stylesheet" href="/styles.css" />
                <link rel="stylesheet" href="/fontawesome/css/regular.min.css" />
                <link rel="stylesheet" href="/fontawesome/css/brands.min.css" />
                <link rel="stylesheet" href="/fontawesome/css/all.min.css" />

                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="" />
                <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@200..800&display=swap" rel="stylesheet" />
            </head>
            <body class="bg-black">
                {props.children}
                <script src="https://unpkg.com/htmx.org@2.0.2"></script>
            </body>
        </html>
    }
}
