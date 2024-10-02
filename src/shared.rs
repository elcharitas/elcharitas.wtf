pub use hypertext::{GlobalAttributes, RenderIterator, Renderable};
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
        const aria_live: Attribute = Attribute; // assertive, polite, off
        const aria_atomic: Attribute = Attribute; // true, false
        const aria_busy: Attribute = Attribute; // true, false
        const aria_controls: Attribute = Attribute;
        const aria_describedby: Attribute = Attribute;
        const aria_disabled: Attribute = Attribute; // true, false
        const aria_dropeffect: Attribute = Attribute;
        const aria_flowto: Attribute = Attribute;
        const aria_grabbed: Attribute = Attribute; // true, false
        const aria_haspopup: Attribute = Attribute; // true, false
    }

    #[allow(non_upper_case_globals)]
    pub trait HtmxAttributes: GlobalAttributes {
        /// issues a `GET` to the specified URL
        const hx_get: Attribute = Attribute;
        /// issues a `POST` to the specified URL
        const hx_post: Attribute = Attribute;
        /// issues a `PUT` to the specified URL
        const hx_put: Attribute = Attribute;
        /// issues a `PATCH` to the specified URL
        const hx_patch: Attribute = Attribute;
        /// issues a `DELETE` to the specified URL
        const hx_delete: Attribute = Attribute;
        /// controls how content will swap in (`outerHTML`, `beforeend`, `afterend`, …)
        const hx_swap: Attribute = Attribute;
        /// specifies the event that triggers the request
        const hx_trigger: Attribute = Attribute;
        /// mark element to swap in from a response (out of band)
        const hx_swap_oob: Attribute = Attribute;
        /// specifies the target element to be swapped
        const hx_target: Attribute = Attribute;
        /// disables htmx processing for the given node and any children nodes
        const hx_disable: Attribute = Attribute;
        /// select content to swap in from a response
        const hx_select: Attribute = Attribute;
        /// push a [`URL`] into the browser location bar to create history
        const hx_push_url: Attribute = Attribute;
        /// replace the URL in the browser location bar
        const hx_replace_url: Attribute = Attribute;
        /// the element to snapshot and restore during history navigation
        const hx_history_elt: Attribute = Attribute;
        /// add progressive enhancement for links and forms
        const hx_boost: Attribute = Attribute;
        /// shows a `prompt()` before submitting a request
        const hx_prompt: Attribute = Attribute;
        /// shows a `confirm()` dialog before issuing a request
        const hx_confirm: Attribute = Attribute;
        /// changes the request encoding type
        const hx_encoding: Attribute = Attribute;
        /// the element to put the `htmx-request` class on during the request
        const hx_indicator: Attribute = Attribute;
        /// add values to submit with the request (JSON format)
        const hx_vals: Attribute = Attribute;
        /// handle `on_click` events with inline scripts on elements
        const hx_on_click: Attribute = Attribute;
        /// handle `on_change` events with inline scripts on elements
        const hx_on_change: Attribute = Attribute;
        /// handle `on_input` events with inline scripts on elements
        const hx_on_input: Attribute = Attribute;
        /// handle `on_submit` events with inline scripts on elements
        const hx_on_submit: Attribute = Attribute;
        /// handle `htmx:on_load` events with inline scripts on elements
        const hx_on__load: Attribute = Attribute;
        /// handle `htmx:on_error` events with inline scripts on elements
        const hx_on__error: Attribute = Attribute;
        /// handle `htmx:before_request` events with inline scripts on elements
        const hx_on__before_request: Attribute = Attribute;
        /// handle `htmx:after_request` events with inline scripts on elements
        const hx_on__after_request: Attribute = Attribute;
    }

    #[allow(non_upper_case_globals)]
    pub trait MetaAttributes: GlobalAttributes {
        const property: Attribute = Attribute;
    }

    elements! {
        svg {
            /// svg width
            width

            /// svg height
            /// e.g. `<svg height="100" />`
            height

            /// svg viewbox
            /// e.g. `<svg viewbox="0 0 100 100" />`
            viewbox

            /// svg fill
            fill

            /// svg stroke
            stroke

            /// svg stroke-width
            stroke_width

            /// svg stroke-linecap
            stroke_linecap

            /// svg stroke-linejoin
            stroke_linejoin

            /// svg stroke-miterlimit
            stroke_miterlimit

            /// svg stroke-dasharray
            stroke_dasharray

            /// xmlns attribute
            xmlns

            /// svg preserveAspectRatio
            preserveAspectRatio

            /// svg version
            version

            /// svg xmlns:xlink
            xmlns_xlink

            /// svg xmlns:sketch
            xmlns_sketch
        }

        circle {
            /// circle cx
            cx

            /// circle cy
            cy

            /// circle r
            r
        }

        line {
            /// line x1
            x1

            /// line x2
            x2

            /// line y1
            y1

            /// line y2
            y2
        }

        path {
            /// path d
            d
        }

        rect {
            /// rect x
            x

            /// rect y
            y

            /// rect width
            width

            /// rect height
            height

            /// rect rx
            rx

            /// rect ry
            ry
        }

        text {
            /// text x
            x

            /// text y
            y
        }

        tspan {
            /// tspan x
            x

            /// tspan y
            y
        }

        def {
            /// def id
            id
        }

        g {
            /// g transform
            transform
        }

        r#use {
            /// use href
            href
        }

        title {
            /// title
            title
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
    impl VoidElement for circle {}
    impl VoidElement for line {}
    impl VoidElement for path {}
    impl VoidElement for rect {}
    impl VoidElement for r#use {}

    impl<T: GlobalAttributes> AriaAttributes for T {}
    impl<T: GlobalAttributes> MetaAttributes for T {}
    impl<T: GlobalAttributes> HtmxAttributes for T {}
}

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

        #[allow(dead_code)]
        impl $name {
            pub fn route_handler(_cx: &mut ngyn::prelude::NgynContext) -> String {
                Self::build()
            }
        }
    };
}
