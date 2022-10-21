use std::{env::var, path::Path};

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&manifest_dir);
    println!(
        "cargo:rustc-link-arg=--script={}",
        manifest_dir.join("linker.ld").display()
    );
}
