fn main() {
    println!("cargo:rustc-link-lib=dylib=audio");
    println!("cargo:rustc-link-search=native=./lib");
}