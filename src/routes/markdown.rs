use std::{
    collections::HashMap,
    sync::LazyLock,
};

use chrono::NaiveDate;
use maud::Markup;
use serde::Deserialize;

use crate::markdown;

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub title: String,
    pub date: Option<NaiveDate>,
    pub tags: Option<Vec<String>>,
}

pub static PAGES: LazyLock<HashMap<String, (Metadata, Markup)>> = LazyLock::new(|| {
    HashMap::from_iter(embed::dir!(".").flatten().iter().filter_map(|file| {
        let path = file.path().as_ref().split_once("routes/").unwrap().1;

        if !path.ends_with(".md") {
            return None;
        }

        let content = String::from_utf8(file.content().to_vec()).unwrap();

        let mut parts = content.splitn(2, "---").skip(1);

        let metadata: Metadata = serde_yaml::from_str(parts.next().unwrap()).unwrap();

        log::info!("Adding page {path}");

        Some((
            path.strip_suffix(".md").unwrap().to_string(),
            (metadata, markdown::parse(parts.next().unwrap())),
        ))
    }))
});
