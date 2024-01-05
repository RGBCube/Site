use maud::{
    html,
    Markup,
};

use crate::{
    asset::Css,
    page::asset,
};

/// Creates a pure HTML CSS and JS cube with 6 faces, the
/// order of the faces are as so:
///
/// front, top, back, bottom, right, left.
pub fn create(title: Option<&str>, css: Css, faces: [Markup; 6]) -> Markup {
    crate::page::create(
        title,
        html! {
            (asset::Css::Shared("cube.css"))
            (css)
        },
        html! {
            .scene {
                .cube {
                    @for (name, content) in ["front", "top", "back", "bottom", "right", "left"].iter().zip(faces) {
                        .(format!("face {name}")) {
                            (content)
                        }
                    }
                }
            }

            (asset::Js::Shared("cube.js"))
        },
    )
}
