use std::{
    collections::HashMap,
    sync::LazyLock,
};

use chrono::{
    DateTime,
    Utc,
};
use maud::Markup;
use serde::Deserialize;

use crate::markdown;

mod ddmmyyyy {
    use chrono::{
        DateTime,
        NaiveDate,
        Utc,
    };
    use serde::{
        self,
        Deserialize,
        Deserializer,
    };

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Option::<String>::deserialize(deserializer)? {
            None => Ok(None),
            Some(s) => {
                Ok(Some(DateTime::<Utc>::from_naive_utc_and_offset(
                    NaiveDate::parse_from_str(&s, "%d/%m/%Y")
                        .map_err(serde::de::Error::custom)?
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    Utc,
                )))
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub title: String,
    pub description: Option<String>,
    #[serde(default, with = "ddmmyyyy")]
    pub date: Option<DateTime<Utc>>,
    pub tags: Option<Vec<String>>,
}

pub static PAGES: LazyLock<HashMap<String, (Metadata, Markup)>> = LazyLock::new(|| {
    HashMap::from_iter(embed::dir!(".").flatten().iter().filter_map(|file| {
        let path = file.path().as_ref().split_once("routes/").unwrap().1;

        if !path.ends_with(".md") {
            return None;
        }

        let content = String::from_utf8(file.content().to_vec()).unwrap();

        let mut parts = content.splitn(3, "---").skip(1);

        let metadata: Metadata = serde_yaml::from_str(parts.next().unwrap()).unwrap();

        log::info!("Adding page {path}");

        Some((
            path.strip_suffix(".md").unwrap().to_string(),
            (metadata, markdown::parse(parts.next().unwrap())),
        ))
    }))
});
