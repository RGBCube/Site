use std::array;

use actix_web::{
    dev::ServiceResponse,
    middleware::ErrorHandlerResponse,
};
use maud::html;

use crate::{
    asset,
    page::cube,
};

pub fn handler<B: 'static>(
    response: ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let (request, response) = response.into_parts();

    let response = response.set_body(
        cube::create(
            asset::Css::Shared("not-found.css"),
            array::from_fn(|_| {
                (html! {
                   div class="frame" { "404" }
                   div class="square black" {}
                   div class="square magenta" {}
                   div class="square magenta" {}
                   div class="square black" {}
                })
                .clone()
            }),
        )
        .into_string(),
    );

    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(request, response)
            .map_into_boxed_body()
            .map_into_right_body(),
    ))
}
