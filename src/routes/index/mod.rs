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
        asset::css::owned!("index.css"),
        [
            html! {
              a href="/contact" {
                div class="frame" {
                  "contact"
                }
              }
            },
            html! {
              a href="/github" {
                div class="frame" {
                  "github"
                }
              }
            },
            html! {},
            html! {},
            html! {},
            html! {},
        ],
    ))
}
