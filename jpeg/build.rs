use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.cc");
    println!("cargo:rustc-link-lib=mozjpeg62");

    let mut build = cc::Build::new();
    build.file("src/wrapper.cc").cpp(true);

    if env::var("TARGET").as_deref() == Ok("wasm32-wasi") {
        build.cpp_link_stdlib(None);
    }

    build.compile("jpeg");
}
