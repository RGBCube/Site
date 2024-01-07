use maud::{
    html,
    Markup,
};

use crate::{
    asset,
    page::cube,
};

pub async fn handler() -> Markup {
    cube::create(
        None,
        asset::css::owned!("index.css"),
        [
            html! {
                a href="/about" {
                    .frame {
                        "about"
                    }
                }
            },
            html! {
                a href="https://github.com/RGBCube" {
                    .frame {
                        "github"
                    }
                }
            },
            html! {},
            html! {},
            html! {
                a href="/contact" {
                    .frame {
                        "contact"
                    }
                }
            },
            html! {
                a href="/blog" {
                    .frame {
                        "blog"
                    }
                }
            },
        ],
    )
}
