extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::path::Path;


fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    #[cfg(windows)]
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("lerc-3.0\\bin\\windows").display());

    #[cfg(target_os="linux")]
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("lerc-3.0/bin/Linux").display());

    #[cfg(target_os="macos")]{
        env::set_var("LLVM_CONFIG_PATH", "/usr/local/opt/llvm/bin/llvm-config"); // For some reason it doesn't pick up the system envs
        println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("lerc-3.0/bin/MacOS").display());
    }

    #[cfg(windows)]
    let include_path = format!("-I{}", Path::new(&dir).join("lerc-3.0\\src\\LercLib\\include").display());

    #[cfg(not(windows))]
    let include_path = format!("-I{}", Path::new(&dir).join("lerc-3.0/src/LercLib/include").display());
    
    println!("cargo:rustc-link-lib=dylib={}", "Lerc");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(include_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
