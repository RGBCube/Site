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
            Some("404"),
            asset::Css::Shared("cube-grid.css"),
            array::from_fn(|_| {
                (html! {
                   .frame {
                        a href="/" { "404" }
                    }
                   .square .black {}
                   .square .magenta {}
                   .square .magenta {}
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
