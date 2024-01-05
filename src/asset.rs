use maud::{
    html,
    Markup,
    PreEscaped,
    Render,
};

use crate::minify;

pub fn extension_of(path: &str) -> Option<&str> {
    path.rsplit_once('.').map(|(_base, extension)| extension)
}

pub enum Js {
    Shared(&'static str),
    Owned(String),
}

impl Render for Js {
    fn render(&self) -> Markup {
        match self {
            Self::Shared(path) => {
                html! {
                    script src=(format!("/assets/{}", minify::insert_min(path))) {}
                }
            },
            Self::Owned(content) => {
                html! {
                    script {
                        (PreEscaped(minify::js(content)))
                    }
                }
            },
        }
    }
}

pub mod js {
    macro_rules! owned {
        ($path:literal) => {
            crate::asset::Js::Owned(::embed::string!($path).to_string())
        };
    }

    pub(crate) use owned;
}

pub enum Css {
    Shared(&'static str),
    Owned(String),
}

impl Render for Css {
    fn render(&self) -> Markup {
        match self {
            Self::Shared(path) => {
                html! {
                    link rel="stylesheet" type="text/css" href=(format!("/assets/{}", minify::insert_min(path)));
                }
            },
            Self::Owned(content) => {
                html! {
                    style {
                        (PreEscaped(minify::css(content)))
                    }
                }
            },
        }
    }
}

pub mod css {
    macro_rules! owned {
        ($path:literal) => {
            crate::asset::Css::Owned(::embed::string!($path).to_string())
        };
    }

    pub(crate) use owned;
}

pub struct File(pub &'static str);

impl Render for File {
    fn render(&self) -> Markup {
        PreEscaped(self.to_string())
    }
}

impl ToString for File {
    fn to_string(&self) -> String {
        format!("/assets/{}", self.0)
    }
}
