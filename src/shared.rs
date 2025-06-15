use lazy_static::lazy_static;
use momenta::nodes::Component;
use ngyn::{prelude::*, shared::server::Transformer};
use serde::{Deserialize, Serialize};

#[derive(Param)]
pub struct PageParams;

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
            name: "Writings",
            href: "/writings"
        },
        NavigationInfo {
            name: "Resume",
            href: "/resume"
        },
        NavigationInfo {
            name: "Adventures",
            href: "/adventures"
        },
    ];
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Post<T> {
    pub slug: String,
    pub title: String,
    pub brief: String,
    pub cover_image: Option<String>,
    pub date: Option<String>,
    pub category: String,
    pub content: Option<String>,
    pub url: Option<String>,
    pub views: Option<u64>,
    pub owner: Option<String>,
    pub branch: Option<String>,
    pub comments: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub url: Option<String>,
    pub image: Option<String>,
    pub tags: Vec<String>,
}

pub fn route_handler<T: Component>(_: T) -> impl Into<RouteHandler>
where
    for<'a> <T as momenta::nodes::Component>::Props: Transformer<'a>,
{
    handler(move |ctx| {
        let body = <T as momenta::nodes::Component>::render(
            &<T as momenta::nodes::Component>::Props::transform(ctx),
        );
        body.to_string()
    })
}
