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
    let contents = embed::bytes!("../../src.tar");
    let mut archive = Archive::new(Cursor::new(contents));

    let mut assets = HashMap::new();

    for entry in archive.entries().unwrap() {
        let mut entry = if let Ok(entry) = entry {
            entry
        } else {
            log::error!("fail");
            continue;
        };

        let path = entry.path_bytes();
        let path = String::from_utf8(path.to_vec()).unwrap();

        if path.ends_with("/") || !ASSET_EXTENSIONS.iter().any(|ext| path.ends_with(ext)) {
            continue;
        }

        let path = path.rsplit_once("/").unwrap_or(("", &path)).1;

        let mut content = Vec::new();
        entry.read_to_end(&mut content).unwrap();

        if minify::is_minifiable(&path) {
            let content = minify::generic(&path, &content);

            log::info!("Minifying asset {path}");
            assets.insert(minify::insert_min(path), Bytes::from(content));
        }

        log::info!("Adding asset {path}");
        assets.insert(path.to_string(), Bytes::from(content));
    }

    assets
});

#[get("/assets/{name}")]
pub async fn handler(name: web::Path<String>) -> HttpResponse {
    let name = name.into_inner();

    if let Some(body) = ASSETS.get(&name) {
        HttpResponse::Ok()
            .content_type(
                mime_guess::from_path(&name)
                    .first_or_octet_stream()
                    .essence_str(),
            )
            .body(Bytes::clone(body))
    } else {
        HttpResponse::NotFound().into()
    }
}
