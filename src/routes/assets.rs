use std::{
    collections::HashMap,
    path,
    sync::LazyLock,
};

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

use super::markdown::PAGES;
use crate::{
    errors::not_found,
    minify,
    page::{
        text,
        Page,
    },
};

const ASSET_EXTENSIONS: &[&str] = &[".js", ".css", ".woff2", ".gif", ".txt"];
const ROOT_EXTENSIONS: &[&str] = &[".txt"];

static ASSETS: LazyLock<HashMap<String, Bytes>> = LazyLock::new(|| {
    let mut assets = HashMap::new();

    for file in embed::dir!("..").flatten() {
        let path = path::Path::new(file.path().as_ref())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        if !ASSET_EXTENSIONS
            .iter()
            .any(|extension| path.ends_with(extension))
        {
            continue;
        }

        let add_asset_prefix = |path: &str| {
            if ROOT_EXTENSIONS
                .iter()
                .any(|extension| path.ends_with(extension))
            {
                path.to_string()
            } else {
                format!("assets/{path}")
            }
        };

        if minify::is_minifiable(path) {
            let content = minify::generic(path, file.content());

            log::info!("Minifying asset {path}");
            assets.insert(
                add_asset_prefix(&minify::insert_min(path)),
                Bytes::from(content),
            );
        }

        log::info!("Adding asset {path}");
        assets.insert(add_asset_prefix(path), Bytes::from(file.content().to_vec()));
    }

    assets
});

pub async fn handler(Path(path): Path<String>) -> Response<Body> {
    if let Some(body) = ASSETS.get(&path) {
        (
            [(
                CONTENT_TYPE,
                mime_guess::from_path(&path)
                    .first_or_octet_stream()
                    .essence_str(),
            )],
            Bytes::clone(body),
        )
            .into_response()
    } else if let Some((metadata, body)) = PAGES.get(&path) {
        text::create(Some(&metadata.title), Page::from_str(&path), body).into_response()
    } else {
        not_found::handler().await.into_response()
    }
}
