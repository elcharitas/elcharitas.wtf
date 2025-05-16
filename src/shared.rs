use ngyn::prelude::*;
use serde::{Deserialize, Serialize};
use simple_rsx::Component;

#[derive(Debug, Default, Serialize, Deserialize)]
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

pub fn route_handler<T: Component>(mut comp: T) -> impl Into<RouteHandler>
where
    <T as simple_rsx::Component>::Props: Default,
{
    let body = comp.render(Default::default());
    handler(move |_| body.to_string())
}
