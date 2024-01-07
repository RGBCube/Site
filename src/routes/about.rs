use maud::Markup;

use crate::{
    markdown,
    page::{
        text,
        Page,
    },
};

pub async fn handler() -> Markup {
    text::create(
        Some("About"),
        Page::About,
        markdown::parse(embed::string!("about.md").as_ref()),
    )
}
