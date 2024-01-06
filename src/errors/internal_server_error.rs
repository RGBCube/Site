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
    );

    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(request, response)
            .map_into_boxed_body()
            .map_into_right_body(),
    ))
}
