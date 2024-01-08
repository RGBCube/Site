use std::array;

use axum::{
    http::StatusCode,
    BoxError,
};
use maud::html;

use crate::{
    asset,
    page::cube,
};

pub async fn handler(_: BoxError) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        cube::create(
            Some("Error"),
            asset::Css::Shared("cube-grid.css"),
            array::from_fn(|_| {
                (html! {
                   .frame { "error" }
                   .square .black {}
                   .square .red {}
                   .square .red {}
                   .square .black {}
                })
                .clone()
            }),
        )
        .into_string(),
    )
}
