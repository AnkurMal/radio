use std::fs;

fn main() {
    fs::copy("lib/audio.dll", "target/debug/audio.dll").unwrap();

    println!("cargo:rustc-link-lib=dylib=audio");
    println!("cargo:rustc-link-search=native=./lib");
}