use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let include = manifest_dir.join("include");
    let nema_include = manifest_dir
        .join("..")
        .join("stm32-bindings-gen")
        .join("nema_gfx")
        .join("include");

    let mut build = cc::Build::new();
    build
        .file(manifest_dir.join("c/nema_hal_baremetal.c"))
        .file(manifest_dir.join("c/bump_alloc.c"))
        .include(&include)
        .include(&nema_include)
        .flag_if_supported("-mfloat-abi=hard")
        .define("USE_HAL_GPU2D_REGISTER_CALLBACKS", "0");

    if env::var("CARGO_FEATURE_STUB").is_ok() {
        build.file(manifest_dir.join("c/gpu2d_hal_stub.c"));
    }

    build.compile("nema_gfx_hal");

    println!("cargo:rerun-if-changed=c/");
    println!("cargo:rerun-if-changed=include/");
}
