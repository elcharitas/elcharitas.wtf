use crate::{graphql_query, requests::GraphQLQuery};

use super::requests::GraphQLClient;
use axum::extract::Query;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PageParams {
    pub slug: String,
}

#[derive(Default, Deserialize)]
pub struct PageQuery {
    pub has_next_page: bool,
    pub cursor: String,
}

impl PageQuery {
    pub fn from_query(query: Query<serde_json::Value>) -> Self {
        let datastar: String = query
            .0
            .get("datastar")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        serde_json::from_str(&datastar).unwrap_or_default()
    }
}

pub struct NavigationInfo {
    pub name: &'static str,
    pub href: &'static str,
}

lazy_static! {
    pub static ref NAVIGATION: Vec<NavigationInfo> = vec![
        NavigationInfo {
            name: "Projects",
            href: "/projects"
        },
        NavigationInfo {
            name: "Essays",
            href: "/essays"
        },
        NavigationInfo {
            name: "Resume",
            href: "/resume"
        },
        NavigationInfo {
            name: "Timeline",
            href: "/adventures"
        },
    ];
    pub static ref HASHNODE_CLIENT: GraphQLClient = GraphQLClient::default();

    // Queries
    pub static ref SINGLE_POST_QUERY: GraphQLQuery = graphql_query!(
        SinglePostByPublication,
        include_str!("../graphql/queries/SinglePostByPublication.graphql")
    );
    pub static ref POSTS_QUERY: GraphQLQuery = graphql_query!(
        PostsByPublication,
        include_str!("../graphql/queries/PostsByPublication.graphql")
    );
    pub static ref RSS_QUERY: GraphQLQuery = graphql_query!(
        RSSFeed,
        include_str!("../graphql/queries/RSSFeed.graphql")
    );
    pub static ref SITEMAP_QUERY: GraphQLQuery = graphql_query!(
        Sitemap,
        include_str!("../graphql/queries/Sitemap.graphql")
    );

    // mutations
    pub static ref NEWSLETTER_MUTATION: GraphQLQuery = graphql_query!(
        SubscribeToNewsletter,
        include_str!("../graphql/mutations/SubscribeToNewsletter.graphql")
    );
}

/// Root query response type
#[derive(Debug, Serialize, Deserialize)]
pub struct PostsByPublicationQuery {
    pub publication: Publication,
}

/// Root query response type for single post
#[derive(Debug, Serialize, Deserialize)]
pub struct SinglePostByPublicationQuery {
    pub publication: Option<SinglePostPublication>,
}

/// Root query response type for RSS feed
#[derive(Debug, Serialize, Deserialize)]
pub struct RSSFeedQuery {
    pub publication: Publication,
}

/// Root query response type for sitemap
#[derive(Debug, Serialize, Deserialize)]
pub struct SitemapQuery {
    pub publication: SitemapPublication,
}

/// Publication type for sitemap query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapPublication {
    pub id: String,
    pub url: String,
    #[serde(rename = "staticPages")]
    pub static_pages: StaticPageConnection,
    pub posts: PublicationPostConnection,
}

/// Static page connection for sitemap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticPageConnection {
    pub edges: Option<Vec<StaticPageEdge>>,
}

/// Static page edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticPageEdge {
    pub node: StaticPage,
}

/// Static page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticPage {
    pub slug: String,
}

/// Publication type for single post query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinglePostPublication {
    pub id: String,
    pub title: String,
    #[serde(rename = "displayTitle")]
    pub display_title: Option<String>,
    pub url: String,
    #[serde(rename = "metaTags")]
    pub meta_tags: Option<String>,
    pub favicon: Option<String>,
    #[serde(rename = "isTeam")]
    pub is_team: bool,
    #[serde(rename = "followersCount")]
    pub followers_count: Option<i32>,
    #[serde(rename = "descriptionSEO")]
    pub description_seo: Option<String>,
    pub posts: PublicationPostConnection,
    pub post: Option<Post>,
    pub author: User,
    #[serde(rename = "ogMetaData")]
    pub og_meta_data: OpenGraphMetaData,
    // pub preferences: Preferences,
    // pub links: Option<PublicationLinks>,
    // pub integrations: Option<PublicationIntegrations>,
}

/// SEO metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEO {
    pub title: Option<String>,
    pub description: Option<String>,
}

/// Content with markdown and html
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Content {
    pub markdown: String,
    pub html: String,
}

/// Post features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostFeatures {
    #[serde(rename = "tableOfContents")]
    pub table_of_contents: TableOfContentsFeature,
}

/// Table of contents feature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableOfContentsFeature {
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    pub items: Vec<TableOfContentsItem>,
}

/// Table of contents item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableOfContentsItem {
    pub id: String,
    pub level: i32,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    pub slug: String,
    pub title: String,
}

/// Post preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPreferences {
    #[serde(rename = "disableComments")]
    pub disable_comments: bool,
}

/// Post comment connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCommentConnection {
    #[serde(rename = "totalDocuments")]
    pub total_documents: i32,
    pub edges: Option<Vec<PostCommentEdge>>,
}

/// Post comment edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCommentEdge {
    pub node: Comment,
}

/// Comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    #[serde(rename = "totalReactions")]
    pub total_reactions: i32,
    pub content: Content,
    pub author: User,
}

/// Publication type
#[derive(Debug, Serialize, Deserialize)]
pub struct Publication {
    pub id: String,
    pub title: String,
    #[serde(rename = "displayTitle")]
    pub display_title: Option<String>,
    pub url: String,
    #[serde(rename = "metaTags")]
    pub meta_tags: Option<String>,
    pub favicon: Option<String>,
    #[serde(rename = "isTeam")]
    pub is_team: bool,
    #[serde(rename = "followersCount")]
    pub followers_count: Option<i32>,
    #[serde(rename = "descriptionSEO")]
    pub description_seo: Option<String>,
    pub posts: PublicationPostConnection,
    pub author: User,
    #[serde(rename = "ogMetaData")]
    pub og_meta_data: OpenGraphMetaData,
    pub preferences: Preferences,
    pub links: Option<PublicationLinks>,
    pub integrations: Option<PublicationIntegrations>,
}

/// Post connection type for pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicationPostConnection {
    #[serde(rename = "totalDocuments")]
    pub total_documents: i32,
    pub edges: Option<Vec<PostEdge>>,
    #[serde(rename = "pageInfo")]
    pub page_info: Option<PageInfo>,
}

/// Post edge for connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostEdge {
    pub node: Post,
}

/// Tag type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub slug: String,
}

/// Post type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub slug: String,
    pub url: String,
    pub brief: String,
    pub title: String,
    pub subtitle: Option<String>,
    #[serde(rename = "publishedAt")]
    pub published_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "readTimeInMinutes")]
    pub read_time_in_minutes: i32,
    #[serde(rename = "reactionCount")]
    pub reaction_count: Option<i32>,
    #[serde(rename = "responseCount")]
    pub response_count: Option<i32>,
    pub views: i32,
    pub seo: Option<SEO>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<PostCoverImage>,
    pub author: Option<User>,
    pub content: Option<Content>,
    #[serde(rename = "ogMetaData")]
    pub og_meta_data: Option<OpenGraphMetaData>,
    pub tags: Vec<Tag>,
    pub comments: Option<PostCommentConnection>,
}

/// User type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub username: Option<String>,
    #[serde(rename = "profilePicture")]
    pub profile_picture: Option<String>,
    #[serde(rename = "followersCount")]
    pub followers_count: Option<i32>,
}

/// Comments connection with total count
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentsConnection {
    #[serde(rename = "totalDocuments")]
    pub total_documents: i32,
}

/// Post cover image
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PostCoverImage {
    pub url: String,
}

/// Page info for pagination
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageInfo {
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: Option<bool>,
}

/// Open Graph metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenGraphMetaData {
    pub image: Option<String>,
}

/// Publication preferences
#[derive(Debug, Serialize, Deserialize)]
pub struct Preferences {
    pub logo: Option<String>,
    #[serde(rename = "darkMode")]
    pub dark_mode: Option<DarkModePreferences>,
    #[serde(rename = "navbarItems")]
    pub navbar_items: Vec<PublicationNavbarItem>,
}

/// Dark mode preferences
#[derive(Debug, Serialize, Deserialize)]
pub struct DarkModePreferences {
    pub logo: Option<String>,
}

/// Publication navbar item
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicationNavbarItem {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: PublicationNavigationType,
    pub label: Option<String>,
    pub url: Option<String>,
}

/// Publication navigation type enum
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PublicationNavigationType {
    Home,
    Page,
    Link,
    #[serde(other)]
    Unknown,
}

/// Publication social links
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicationLinks {
    pub twitter: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
    pub hashnode: Option<String>,
}

/// Publication integrations
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicationIntegrations {
    #[serde(rename = "umamiWebsiteUUID")]
    pub umami_website_uuid: Option<String>,
    #[serde(rename = "gaTrackingID")]
    pub ga_tracking_id: Option<String>,
    #[serde(rename = "fbPixelID")]
    pub fb_pixel_id: Option<String>,
    #[serde(rename = "hotjarSiteID")]
    pub hotjar_site_id: Option<String>,
    #[serde(rename = "matomoURL")]
    pub matomo_url: Option<String>,
    #[serde(rename = "matomoSiteID")]
    pub matomo_site_id: Option<String>,
    #[serde(rename = "fathomSiteID")]
    pub fathom_site_id: Option<String>,
    #[serde(rename = "fathomCustomDomain")]
    pub fathom_custom_domain: Option<String>,
    #[serde(rename = "fathomCustomDomainEnabled")]
    pub fathom_custom_domain_enabled: Option<bool>,
    #[serde(rename = "plausibleAnalyticsEnabled")]
    pub plausible_analytics_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub url: String,
    pub image: Option<String>,
    pub tags: Vec<String>,
    pub stargazers_count: f32,
    pub language: Option<String>,
    pub updated_at: String,
    pub homepage: String,
    pub watching: f32,
}

pub mod xml_elements {
    pub mod dom {
        pub mod elements {
            use momenta::derive_elements;
            extern crate alloc;

            derive_elements! {
                rss {}
                channel {}
                loc {}
                title {}
                description {}
                link {}
                lastmod {}
                changefreq {}
                priority {}
                image {}
                lastBuildDate {}
                item {}
                guid {}
                pubDate {}
                sitemapindex {
                    xmlns: String,
                }
                sitemap {}

            }
        }
    }
}
