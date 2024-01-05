use actix_web::get;
use maud::Markup;

use crate::{
    markdown,
    page::{
        text,
        Page,
    },
};

#[get("/about")]
pub async fn handler() -> actix_web::Result<Markup> {
    Ok(text::create(
        Some("About"),
        Page::About,
        markdown::parse(embed::string!("about.md").as_ref()),
    ))
}
