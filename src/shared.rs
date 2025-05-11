use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

#[macro_export]
/// works simply in this manner:
/// ```rust
/// derive_props! {
///    PostProps {
///       title: String,
///       date: Option<String>,
///      comments: Option<u8>,
///    }
/// }
/// ```
macro_rules! derive_props {
    ($name:ident {
        $($prop:ident: $type:ty,)*
    }) => {
        #[derive(serde::Deserialize, serde::Serialize)]
        pub struct $name {
            $(pub $prop: $type,)*
        }
    };
}

#[macro_export]
macro_rules! derive_component {
    ($visibility:vis $name:ident($props_ident:ident: $props:ident) {
        $($body:tt)*
    }) => {
        $visibility struct $name;

        #[allow(dead_code)]
        impl $name {

            pub fn build($props_ident: $props) -> String {
                let output = {
                    $($body)*
                };
                output.to_string()
            }
        }
    };
    ($visibility:vis $name:ident {
        $($body:tt)*
    }) => {
        $visibility struct $name;

        impl $name {
            pub fn render() -> String {
                let output = {
                    $($body)*
                };
                output.to_string()
            }
        }

        #[allow(dead_code)]
        impl $name {
            pub fn route_handler(_cx: &mut ngyn::prelude::NgynContext) -> String {
                Self::render()
            }
        }
    };
}
