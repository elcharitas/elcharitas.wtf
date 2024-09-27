pub use hypertext::{GlobalAttributes, RenderIterator, Renderable, Rendered};
use ngyn::shared::server::{Bytes, ToBytes};

pub use html_elements::AriaAttributes;
use serde::{Deserialize, Serialize};

pub mod html_elements {
    pub use hypertext::html_elements::*;
    use hypertext::{Attribute, GlobalAttributes, VoidElement};

    #[allow(non_upper_case_globals)]
    pub trait AriaAttributes: GlobalAttributes {
        const aria_label: Attribute = Attribute;
        const aria_labelledby: Attribute = Attribute;
        const aria_hidden: Attribute = Attribute;
    }

    pub struct Link;

    #[allow(non_upper_case_globals)]
    impl Link {
        // sets the `href` to target the given URL
        // e.g. `<Link href="https://example.com">...</Link>`
        pub const href: Attribute = Attribute;
    }

    pub struct FontAwesomeIcon;

    #[allow(non_upper_case_globals)]
    impl FontAwesomeIcon {
        // sets the icon to the given class
        // e.g. `<FontAwesomeIcon icon="fas fa-home" />`
        pub const icon: Attribute = Attribute;
    }

    impl GlobalAttributes for Link {}
    impl GlobalAttributes for FontAwesomeIcon {}
    impl VoidElement for FontAwesomeIcon {}

    impl<T: GlobalAttributes> AriaAttributes for T {}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post<T> {
    pub slug: String,
    pub title: String,
    pub brief: String,
    pub cover_mage: Option<String>,
    pub date: Option<String>,
    pub category: String,
    pub content: Option<String>,
    pub url: Option<String>,
    pub views: Option<u64>,
    pub owner: Option<String>,
    pub branch: Option<String>,
    pub comments: Option<T>,
}

pub struct Rsx<T = fn(&mut String)>(pub T);

impl<T> From<T> for Rsx<T>
where
    T: Fn(&mut String),
{
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl Into<String> for Rsx<fn(&mut String)> {
    fn into(self) -> String {
        let mut output = String::new();
        (self.0)(&mut output);
        output
    }
}

impl<T> ToBytes for Rsx<T>
where
    T: Fn(&mut String),
{
    fn to_bytes(self) -> Bytes {
        let output = self.0.render();
        output.as_inner().as_str().to_bytes()
    }
}

#[macro_export]
macro_rules! rsx_move {
    () => {
        hypertext::rsx_move! {}
    };
    ($variable_name:ident) => {
        (hypertext::rsx_move! {
            $variable_name
        }) as fn(&mut String)
    };
    // match the pattern `variable_name &&`
    ($variable_name:expr => $($rest:tt)*) => {
        if $variable_name {
            Some(hypertext::rsx_move! {
                $($rest)*
            })
        } else {
           None
        }
    };
    // match the pattern `variable_name ? some data :`
    ($variable_name:expr => $($some_data:tt)* => $($rest:tt)*) => {
        if $variable_name {
            hypertext::rsx_move! {
                $($some_data)*
            }
        } else {
            hypertext::rsx_move! {
                $($rest)*
            }
        }
    };
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
/// works simply in this manner:
/// ```rust
/// derive_component! {
///   Post: PostProps {
///   let PostProps { title, date, comments } = props;
///   Rsx(hypertext::rsx! {
///    <article>
///     <h1>{title}</h1>
///    {rsx_move! {date.is_some() => (
///    <time datetime={date.unwrap()}>{date.unwrap()}</time>
///   )}}
///   </article>
///  })
/// }
/// }
/// ```
macro_rules! derive_component {
    ($visibility:vis $name:ident($props_ident:ident: $props:ident) {
        $($body:tt)*
    }) => {
        $visibility struct $name;

        impl $name {
            pub fn render(raw_props: &str) -> impl FnOnce(&mut String) {
                let $props_ident: $props = serde_json::from_str(raw_props).expect("failed to parse props");
                $($body)*
            }
        }
    };
}
