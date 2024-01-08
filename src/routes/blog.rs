use std::sync::LazyLock;

use axum::extract::Path;
use indexmap::IndexMap;
use itertools::Itertools;
use maud::{
    html,
    Markup,
};

use super::markdown::{
    Metadata,
    PAGES,
};
use crate::{
    errors::not_found,
    page::{
        text,
        Page,
    },
};

static ENTRIES: LazyLock<IndexMap<&'static str, (&'static Metadata, Markup)>> =
    LazyLock::new(|| {
        IndexMap::from_iter(
            PAGES
                .iter()
                .sorted_by(|(_, a), (_, b)| b.0.date.cmp(&a.0.date))
                .filter_map(|(path, (metadata, body))| {
                    if let Some(name) = path.strip_prefix("blog/") {
                        let body = html! {
                            (body)

                            @if let Some(tags) = &metadata.tags {
                                p {
                                    "Tags: "
                                    (tags.join(", "))
                                }
                            }
                        };

                        Some((name, (metadata, body)))
                    } else {
                        None
                    }
                }),
        )
    });

pub async fn index_handler() -> Markup {
    text::create(
        Some("Blog"),
        Page::Blog,
        &html! {
            h1 { "Blog Articles" }
            p { "RSS feed coming soon, probably :)" }

            ul {
                @for (path, (metadata, ..)) in ENTRIES.iter() {
                    li {
                        (metadata.date.unwrap().format("%d/%m/%Y"))
                        " - "
                        a href=(format!("/blog/{path}")) {
                            (metadata.title)
                        }
                    }
                }
            }
        },
    )
}

pub async fn entry_handler(Path(entry): Path<String>) -> Markup {
    if let Some((metadata, body)) = ENTRIES.get(entry.as_str()) {
        text::create(Some(&metadata.title), Page::Other, body)
    } else {
        not_found::handler().await
    }
}
