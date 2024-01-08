use std::str::from_utf8;

use minify_js::{
    Session,
    TopLevelMode,
};

use crate::asset::extension_of;

pub const MINIFIABLE: &[&str] = &[".js", ".css"];

pub fn is_minifiable(path: &str) -> bool {
    MINIFIABLE.iter().any(|extension| path.ends_with(extension))
}

pub fn insert_min(path: &str) -> String {
    match path.rsplit_once('.') {
        Some((base, extension)) => format!("{base}.min.{extension}"),
        None => format!("{path}.min"),
    }
}

pub fn generic(path: &str, content: &[u8]) -> Vec<u8> {
    match extension_of(path) {
        Some("js") => js(from_utf8(content).unwrap()).into_bytes(),
        Some("css") => css(from_utf8(content).unwrap()).into_bytes(),
        _ => content.to_vec(),
    }
}

pub fn js(content: &str) -> String {
    let mut output = Vec::new();

    minify_js::minify(
        &Session::new(),
        TopLevelMode::Module,
        content.as_bytes(),
        &mut output,
    )
    .unwrap();

    String::from_utf8(output).unwrap()
}

pub fn css(content: &str) -> String {
    // TODO
    content.to_string()
}
