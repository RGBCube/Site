use actix_web::get;
use maud::{
    html,
    Markup,
};

use crate::{
    asset,
    page::cube,
};

#[get("/")]
pub async fn handler() -> actix_web::Result<Markup> {
    Ok(cube::create(
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
    ))
}
