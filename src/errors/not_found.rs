use std::array;

use axum::{
    http::StatusCode,
    response::IntoResponse,
};
use maud::html;

use crate::{
    asset,
    page::cube,
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
                        a href="/" { "404" }
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
