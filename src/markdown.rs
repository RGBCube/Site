use maud::{
    Markup,
    PreEscaped,
};
use pulldown_cmark::{
    html,
    Options,
    Parser,
};

pub fn parse(markdown: &str) -> Markup {
    let mut output = String::new();

    let parser = Parser::new_ext(markdown, Options::all());
    html::push_html(&mut output, parser);

    PreEscaped(output)
}
