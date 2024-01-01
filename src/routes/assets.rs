use std::{
    collections::HashMap,
    io::{
        Cursor,
        Read,
    },
    sync::LazyLock,
};

use actix_web::{
    get,
    web,
    HttpResponse,
};
use bytes::Bytes;
use tar::Archive;

use crate::minify;

const ASSET_EXTENSIONS: &[&str] = &[".js", ".css", ".woff2", ".gif"];

static ASSETS: LazyLock<HashMap<String, Bytes>> = LazyLock::new(|| {
    let mut assets = HashMap::new();

    for file in embed::dir!("../..").flatten() {
        let path = file.path.file_name().unwrap().to_str().unwrap();

        if !ASSET_EXTENSIONS
            .iter()
            .any(|extension| path.ends_with(extension))
        {
            continue;
        }

        if minify::is_minifiable(path) {
            let content = minify::generic(path, &file.content);

            log::info!("Minifying asset {path}");
            assets.insert(minify::insert_min(path), Bytes::from(content));
        }

        log::info!("Adding asset {path}");
        assets.insert(path.to_string(), Bytes::from(file.content));
    }

    assets
});

#[get("/assets/{path}")]
pub async fn handler(path: web::Path<String>) -> HttpResponse {
    let path = path.into_inner();

    if let Some(body) = ASSETS.get(&path) {
        HttpResponse::Ok()
            .content_type(
                mime_guess::from_path(&path)
                    .first_or_octet_stream()
                    .essence_str(),
            )
            .body(Bytes::clone(body))
    } else {
        HttpResponse::NotFound().into()
    }
}
