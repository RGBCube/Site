use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=src.tar");

    Command::new("tar")
        .args(["-czf", "src.tar", "src"])
        .output()
        .expect("Failed to create tar archive");
}
