pub use hypertext::{GlobalAttributes, RenderIterator, Renderable, Rendered};
use ngyn::shared::server::{Bytes, ToBytes};

pub use html_elements::AriaAttributes;
pub use html_elements::MetaAttributes;
use serde::{Deserialize, Serialize};

pub mod html_elements {
    pub use hypertext::html_elements::*;
    use hypertext::{elements, Attribute, GlobalAttributes, VoidElement};

    #[allow(non_upper_case_globals)]
    pub trait AriaAttributes: GlobalAttributes {
        const aria_label: Attribute = Attribute;
        const aria_labelledby: Attribute = Attribute;
        const aria_hidden: Attribute = Attribute;
    }

    #[allow(non_upper_case_globals)]
    pub trait MetaAttributes: GlobalAttributes {
        const property: Attribute = Attribute;
    }

    elements! {
        Link {
            // sets the `href` to target the given URL
            // e.g. `<Link href="https://example.com">...</Link>`
            href

            // sets the `target` to the given value
            // e.g. `<Link target="_blank">...</Link>`
            target
        }

        FontAwesomeIcon {
            // sets the icon to the given class
            // e.g. `<FontAwesomeIcon icon="fas fa-home" />`
            icon

            // sets the size of the icon
            // e.g. `<FontAwesomeIcon size="24" />`
            // e.g. `<FontAwesomeIcon size="2x" />`
            size
        }
    }

    impl VoidElement for FontAwesomeIcon {}

    impl<T: GlobalAttributes> AriaAttributes for T {}
    impl<T: GlobalAttributes> MetaAttributes for T {}
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

impl<T> Rsx<T>
where
    T: FnOnce(&mut String),
{
    pub fn inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Rsx<T>
where
    T: FnOnce(&mut String),
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

impl Serialize for Rsx<fn(&mut String)> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let output = self.0.render();
        serializer.serialize_str(output.as_inner().as_str())
    }
}

impl<'de> Deserialize<'de> for Rsx<fn(&mut String)> {
    fn deserialize<D>(_deserializer: D) -> Result<Rsx<fn(&mut String)>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Rsx(move |_| {}))
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

        #[allow(dead_code)]
        impl $name {
            pub fn render(raw_props: &str) -> impl FnOnce(&mut String) {
                let $props_ident: $props = serde_json::from_str(raw_props).expect("failed to parse props");
                $($body)*
            }

            pub fn with($props_ident: $props) -> impl FnOnce(&mut String) {
                $($body)*
            }

            pub fn build($props_ident: $props) -> String {
                Self::with($props_ident).render().as_inner().to_string()
            }
        }
    };
    ($visibility:vis $name:ident {
        $($body:tt)*
    }) => {
        $visibility struct $name;

        #[allow(dead_code)]
        impl $name {
            pub fn render(_raw_props: &str) -> impl FnOnce(&mut String) {
                $($body)*
            }

            pub fn with() -> impl FnOnce(&mut String) {
                $($body)*
            }

            pub fn build() -> String {
                Self::with().render().as_inner().to_string()
            }
        }

        impl $name {
            pub fn route_handler(_cx: &mut ngyn::prelude::NgynContext) -> String {
                Self::build()
            }
        }
    };
}
