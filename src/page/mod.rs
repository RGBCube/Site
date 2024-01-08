pub mod cube;
mod elements;
pub mod text;

use std::sync::LazyLock;

use anyhow::Context;
use cargo_toml::Manifest;
pub use elements::*;
use maud::{
    html,
    Markup,
    DOCTYPE,
};

use crate::asset;

pub static MANIFEST: LazyLock<Manifest> = LazyLock::new(|| {
    Manifest::from_str(&embed::string!("../../Cargo.toml"))
        .with_context(|| "Failed to deserialize Cargo manifest")
        .unwrap()
});

/// Enum used to incidate which page we are on.
#[allow(dead_code)]
pub enum Page {
    Home,
    About,
    Blog,
    Contact,
    Other,
}

impl Page {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::About => "about",
            Self::Blog => "blog",
            Self::Contact => "contact",
            Self::Other => "other",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "home" => Self::Home,
            "about" => Self::About,
            "blog" => Self::Blog,
            "contact" => Self::Contact,
            _ => Self::Other,
        }
    }
}
/// Creates a page with the given head and body.
///
/// This is the most low level function for page creation
/// as all pages use this, as this function provides the
/// page title, OpenGraph and other information.
pub fn create(title: Option<&str>, head: Markup, body: Markup) -> Markup {
    html! {
        (DOCTYPE)

        head {
            meta charset="UTF-8";

            (pname("viewport", "width=device-width, initial-scale=1.0"))
            (property("og:type", "website"))

            @let name = &MANIFEST.package.as_ref().unwrap().authors()[0];

            title { ({
                if let Some(title) = title {
                    format!("{title} - {name}")
                } else {
                    name.clone()
                }
            }) }
            (pname("author", name))

            (property("og:site_name", name))
            (property("og:title", name))

            @let description = MANIFEST.package.as_ref().unwrap().description().unwrap();
            (pname("description", description))
            (property("og:description", description))

            link rel="icon" href=(asset::File("icon.gif")) type="image/gif";

            (property("og:image", &asset::File("thumbnail.png").to_string()))
            (property("og:image:type", "image/png"))
            (property("og:image:height", "1080"))
            (property("og:image:width", "600"))

            @let url = MANIFEST.package.as_ref().unwrap().homepage().unwrap();
            (property("og:url", url))
            link rel="canonical" href=(url);

            (asset::Css::Shared("page.css"))
            (head)
        }

        body {
            (body)
        }
    }
}
