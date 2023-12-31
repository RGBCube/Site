use std::{
    env::temp_dir,
    fs::File,
    hash::{
        BuildHasher,
        RandomState,
    },
    io::Write,
};

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
        Some("js") => js(&String::from_utf8(content.to_vec()).unwrap()).into_bytes(),
        Some("css") => css(&String::from_utf8(content.to_vec()).unwrap()).into_bytes(),
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

    String::from_utf8(output)
        .map_err(|error| {
            let hash = RandomState::new()
                .hash_one(error.clone().into_bytes())
                .to_string();

            let path = temp_dir().join(hash);

            let mut file = File::create(&path).unwrap();
            file.write_all(&error.into_bytes()).unwrap();

            format!(
                "Failed to create a String from minified JavaScript code. The minified code has \
                 been written to {}",
                path.display()
            )
        })
        .unwrap()
}

pub fn css(content: &str) -> String {
    // TODO
    content.to_string()
}
