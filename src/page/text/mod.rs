use chrono::{
    Datelike,
    Local,
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
pub fn create(title: Option<&str>, page: Page, body: &Markup) -> Markup {
    crate::page::create(
        title,
        html! {
            (asset::Css::Shared("text.css"))
            (asset::Css::Owned(format!(r"
                .{page} {{
                    font-style: italic;
                }}
            ", page = page.as_str()).into()))
        },
        html! {
            .centered {
                nav {
                    .content {
                        a.home href="/" { "HOME" }
                        a.about href="/about" { "ABOUT" }
                        a.blog href="/blog" { "BLOG" }
                        a.contact href="/contact" { "CONTACT" }
                    }
                }

                (body)

                footer {
                    "Copyright © "
                    (Local::now().year())
                    " RGBCube"
                }
            }
        },
    )
}
