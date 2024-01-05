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
pub fn create(title: Option<&str>, page: Page, body: Markup) -> Markup {
    crate::page::create(
        title,
        html! {
            (asset::Css::Shared("text.css"))
            (asset::Css::Owned(format!(r"
                .{page} {{
                    font-style: italic !important;
                }}
            ", page = page.as_str())))
        },
        html! {
            .not-flex {
                nav {
                    a.home href="/" { "HOME" }
                    a.about href="/about" { "ABOUT" }
                    a.blog href="/blog" { "BLOG" }
                    a.contact href="/contact" { "CONTACT" }
                }

                (body)

                footer {
                    "Served by "
                    (env::current_exe().unwrap_or_else(|_| PathBuf::from("the toaster in my bathtub")).display())
                }
            }
        },
    )
}
