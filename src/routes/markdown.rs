use std::{
    collections::HashMap,
    path,
    sync::LazyLock,
};

use axum::extract::Path;
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

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub title: String,
    pub date: Option<NaiveDate>,
    pub tags: Option<Vec<String>>,
}

pub static PAGES: LazyLock<HashMap<String, (Metadata, Markup)>> = LazyLock::new(|| {
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
            (metadata, markdown::parse(content)),
        );
    }

    pages
});

pub async fn handler(Path(path): Path<String>) -> Markup {
    if let Some((metadata, body)) = PAGES.get(&path) {
        text::create(Some(&metadata.title), Page::from_str(&path), body)
    } else {
        not_found::handler().await
    }
}
