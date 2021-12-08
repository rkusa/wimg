fn main() {
    cbindgen::Builder::new()
        .with_crate(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("wimg.h");
}
