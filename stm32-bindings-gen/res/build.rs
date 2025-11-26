use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    println!(
        "cargo:rustc-link-search=native={}",
        crate_dir.join("src/lib").to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=wba_mac_lib");
}
