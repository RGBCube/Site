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
        StatusCode,
    },
    response::IntoResponse,
};
use bytes::Bytes;

use crate::minify;

const ASSET_EXTENSIONS: &[&str] = &[".js", ".css", ".woff2", ".gif"];

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

        if minify::is_minifiable(path) {
            let content = minify::generic(path, file.content());

            log::info!("Minifying asset {path}");
            assets.insert(minify::insert_min(path), Bytes::from(content));
        }

        log::info!("Adding asset {path}");
        assets.insert(path.to_string(), Bytes::from(file.content().to_vec()));
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
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
