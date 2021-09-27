extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let lib_dir = env::var("LERC_LIB_DIR").expect("LERC_LIB_DIR");
    let include_dir = env::var("LERC_INCLUDE_DIR").expect("LERC_INCLUDE_DIR");

    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=dylib=Lerc");
    println!("cargo:rerun-if-changed=wrapper.h");

    let include_path = format!("-I{}", include_dir);

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(include_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
