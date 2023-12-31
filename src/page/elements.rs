use maud::{
    html,
    Markup,
};

/// Creates a meta tag with property and content.
pub fn property(name: &str, content: &str) -> Markup {
    html! {
        meta property=(name) content=(content);
    }
}

/// Creates a meta tag with name and content.
pub fn pname(name: &str, content: &str) -> Markup {
    html! {
        meta name=(name) content=(content);
    }
}
