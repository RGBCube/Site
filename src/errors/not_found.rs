use std::array;

use axum::{
    http::StatusCode,
    response::IntoResponse,
};
use maud::html;

use crate::{
    asset,
    page::{
        cube,
        MANIFEST,
    },
};

pub async fn handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        cube::create(
            Some("404"),
            asset::Css::Shared("cube-grid.css"),
            array::from_fn(|_| {
                html! {
                   .frame {
                        a href=(MANIFEST.package.as_ref().unwrap().homepage().unwrap()) { "404" }
                    }
                   .square .black {}
                   .square .magenta {}
                   .square .magenta {}
                   .square .black {}
                }
            }),
        ),
    )
}
