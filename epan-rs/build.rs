extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=wireshark");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .hide_type("max_align_t")
        .clang_arg("-I/usr/include/wireshark")
        .clang_arg("-I/usr/include/glib-2.0")
        .clang_arg("-I/usr/lib64/glib-2.0/include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
