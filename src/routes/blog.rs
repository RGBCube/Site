use axum::extract::Path;
use maud::{
    html,
    Markup,
};

use super::markdown::PAGES;
use crate::{
    errors::not_found,
    page::{
        text,
        Page,
    },
};

pub async fn index_handler() -> Markup {
    text::create(
        Some("blog"),
        Page::Blog,
        &html! {
            h1 { "Blog Articles" }
            p { "RSS feed coming soon, probably :)" }

            ul {
                @let pages = &*PAGES;
                @for (path, (metadata, ..)) in pages.iter() {
                    @if path.starts_with("blog") {
                        li {
                            (metadata.date.unwrap().format("%d/%m/%Y"))
                            " - "
                            a href=(format!("/{path}")) {
                                (metadata.title)
                            }
                        }
                    }
                }
            }
        },
    )
}

pub async fn entry_handler(Path(path): Path<String>) -> Markup {
    if let Some((metadata, body)) = PAGES.get(&path) {
        text::create(
            Some(&metadata.title),
            Page::Other,
            &html! {
                (body)

                @if let Some(tags) = &metadata.tags {
                    p {
                        "Tags: "
                        (tags.join(", "))
                    }
                }
            },
        )
    } else {
        not_found::handler().await
    }
}
