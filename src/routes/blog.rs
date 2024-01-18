use std::sync::LazyLock;

use axum::{
    body::Body,
    extract::Path,
    http::{
        header::CONTENT_TYPE,
        Response,
    },
    response::IntoResponse,
};
use bytes::Bytes;
use chrono::{
    Datelike,
    Utc,
};
use indexmap::IndexMap;
use itertools::Itertools;
use maud::{
    html,
    Markup,
};
use rss::{
    CategoryBuilder,
    ChannelBuilder,
    ItemBuilder,
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
        MANIFEST,
    },
};

static ENTRIES: LazyLock<IndexMap<&'static str, (&'static Metadata, Markup)>> = LazyLock::new(
    || {
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

                            p {
                                "Also, if you are a dinosaur that enjoys good technogoly, check out my"
                                a href="/feed" { "RSS Feed" }
                                "."
                            }
                        };

                        Some((name, (metadata, body)))
                    } else {
                        None
                    }
                }),
        )
    },
);

pub async fn index_handler() -> Markup {
    text::create(
        Some("Blog"),
        Page::Blog,
        &html! {
            h1 { "Blog Articles" }
            p {
                "Are you old? Then you might want to check out the super cool "
                a href="/feed" { "RSS Feed" }
                "."
            }

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

pub async fn entry_handler(Path(entry): Path<String>) -> Response<Body> {
    if let Some((metadata, body)) = ENTRIES.get(entry.as_str()) {
        text::create(Some(&metadata.title), Page::Other, body).into_response()
    } else {
        not_found::handler().await.into_response()
    }
}

static FEED: LazyLock<Bytes> = LazyLock::new(|| {
    let url = MANIFEST.package.as_ref().unwrap().homepage().unwrap();

    let items = ENTRIES.iter().map(|(path, (metadata, body))| {
        ItemBuilder::default()
            .link(Some(format!("{url}{path}")))
            .title(Some(metadata.title.clone()))
            .description(metadata.description.clone())
            .author(Some("contact@rgbcu.be".to_string()))
            .categories(
                metadata
                    .tags
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|tag| CategoryBuilder::default().name(tag.clone()).build())
                    .collect_vec(),
            )
            .pub_date(metadata.date.map(|date| date.to_rfc2822()))
            .content(Some(body.clone().into_string()))
            .build()
    });

    let channel = ChannelBuilder::default()
        .title("RGBCube's Blog".to_string())
        .link(format!("{url}blog"))
        .description(
            "The webpage where RGBCube puts his schizophrenic rambling about software and all the \
             likes"
                .to_string(),
        )
        .copyright(Some(format!("Copyright © {} RGBCube", Utc::now().year())))
        .language(Some("en-us".to_string()))
        .webmaster(Some("contact@rgbcu.be".to_string()))
        .items(items.collect_vec())
        .build();

    Bytes::from(channel.to_string().into_bytes())
});

pub async fn feed_handler() -> impl IntoResponse {
    ([(CONTENT_TYPE, "application/xml")], Bytes::clone(&*FEED))
}
