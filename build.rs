extern crate wayland_scanner;

use std::env::var;
use std::path::Path;

use wayland_scanner::{generate_code, Side};

fn main() {
    // Location of the xml file, relative to the `Cargo.toml`
    let protocol_file = "./protocols/input-method-unstable-v2.xml";

    // Target directory for the generate files
    let out_dir_str = var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);

    generate_code(
        protocol_file,
        out_dir.join("input_method_api.rs"),
        Side::Client,
    );
    println!("cargo:rerun-if-changed=./protocols/");
    println!("cargo:rerun-if-changed=build.rs");
}
