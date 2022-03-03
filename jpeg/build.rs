use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.c");
    println!("cargo:rustc-link-lib=mozjpeg62");

    let mut build = cc::Build::new();
    build.file("src/wrapper.c");

    if env::var("TARGET").as_deref() == Ok("wasm32-wasi") {
        build.define("WASM", None);
    }

    build.compile("jpeg");
}
