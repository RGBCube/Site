use std::{
    collections::HashMap,
    path,
    sync::LazyLock,
};

use axum::{
    body::Body,
    extract::Path,
    http::{
        Response,
        StatusCode,
    },
    response::{
        Html,
        IntoResponse,
    },
};
use maud::Markup;

use crate::{
    markdown,
    page::{
        text,
        Page,
    },
};

static PAGES: LazyLock<HashMap<String, Markup>> = LazyLock::new(|| {
    let mut pages = HashMap::new();

    for file in embed::dir!(".").flatten() {
        let path = path::Path::new(file.path().as_ref())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        if !path.ends_with(".md") {
            continue;
        }

        log::info!("Adding page {path}");
        pages.insert(
            path.to_string().strip_suffix(".md").unwrap().to_string(),
            markdown::parse(&String::from_utf8(file.content().to_vec()).unwrap()),
        );
    }

    pages
});

pub async fn handler(Path(path): Path<String>) -> Response<Body> {
    if let Some(body) = PAGES.get(&path) {
        Html(text::create(Some("test"), Page::from_str(&path), &body).into_string()).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
