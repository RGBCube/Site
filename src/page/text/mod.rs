use std::{
    env,
    path::PathBuf,
};

use maud::{
    html,
    Markup,
};

use crate::page::{
    asset,
    Page,
};

/// Creates a simple text page.
pub fn create(page: Page, title: &str, body: Markup) -> Markup {
    crate::page::create(
        html! {
            (asset::Css::Shared("text.css"))
        },
        html! {
            nav {
                a href="/" { "HOME" }
                a href="/about" { "ABOUT" }
                a href="/blog" { "BLOG" }
                a href="/contact" { "CONTACT" }

                span.title { (title) }
            }

            (body)

            footer {
                "Served by "
                (env::current_exe().unwrap_or_else(|_| PathBuf::from("asd")).display())
            }
        },
    )
}
