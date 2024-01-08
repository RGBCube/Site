use std::array;

use maud::{
    html,
    Markup,
};

use crate::{
    asset,
    page::cube,
};

pub async fn handler() -> Markup {
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
    )
}
