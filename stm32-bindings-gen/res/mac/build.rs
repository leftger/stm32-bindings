use std::path::{Path, PathBuf};
use std::{env, fs, io};

fn add_dir(dir: &Path) -> io::Result<()> {
    if !dir.exists() {
        return Ok(());
    }

    println!("cargo:rustc-link-search=native={}", dir.display());

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            add_dir(&path)?;
        }
    }

    Ok(())
}

fn main() {
    let crate_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let lib_dir = crate_dir.join("src").join("lib");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
    let is_embedded = target_os == "none" || target_family == "embedded";

    if !is_embedded {
        return;
    }

    add_dir(&lib_dir).expect("failed to add link search paths");

    env::vars()
        .filter_map(|(key, _)| {
            key.strip_prefix("CARGO_FEATURE_LIB_")
                .map(|suffix| suffix.to_ascii_lowercase())
        })
        .for_each(|lib| println!("cargo:rustc-link-lib=static={lib}"));
}
