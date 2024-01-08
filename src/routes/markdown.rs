use std::{
    collections::HashMap,
    path,
    sync::LazyLock,
};

use axum::{
    body::Body,
    extract::Path,
    http::Response,
    response::{
        Html,
        IntoResponse,
    },
};
use chrono::NaiveDate;
use maud::Markup;
use serde::Deserialize;

use crate::{
    errors::not_found,
    markdown,
    page::{
        text,
        Page,
    },
};

#[derive(Deserialize)]
struct Metadata {
    title: String,
    // TODO: Use these for blog articles.
    #[allow(dead_code)]
    date: Option<NaiveDate>,
    #[allow(dead_code)]
    tags: Option<Vec<String>>,
}

static PAGES: LazyLock<HashMap<String, (Metadata, Markup)>> = LazyLock::new(|| {
    let routes_path = path::Path::new(file!())
        .parent()
        .unwrap()
        .canonicalize()
        .unwrap();

    let mut pages = HashMap::new();

    for file in embed::dir!(".").flatten() {
        let path = path::Path::new(file.path().as_ref())
            .strip_prefix(&routes_path)
            .unwrap()
            .to_str()
            .unwrap();

        if !path.ends_with(".md") {
            continue;
        }

        let content = String::from_utf8(file.content().to_vec()).unwrap();

        let (metadata, content) = content.split_once("---").unwrap();

        let metadata: Metadata = serde_yaml::from_str(metadata).unwrap();

        log::info!("Adding page {path}");
        pages.insert(
            path.to_string().strip_suffix(".md").unwrap().to_string(),
            (metadata, markdown::parse(&content)),
        );
    }

    pages
});

pub async fn handler(Path(path): Path<String>) -> Response<Body> {
    if let Some((metadata, body)) = PAGES.get(&path) {
        Html(text::create(Some(&metadata.title), Page::from_str(&path), &body).into_string())
            .into_response()
    } else {
        not_found::handler().await.into_response()
    }
}
