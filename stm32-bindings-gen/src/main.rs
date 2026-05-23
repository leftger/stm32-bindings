use std::path::PathBuf;

use stm32_bindings_gen::{Gen, Options};

fn main() {
    let build_dir = PathBuf::from("build/stm32-bindings");
    let sources_dir = PathBuf::from("sources");

    let opts = Options {
        build_dir,
        sources_dir,
    };

    Gen::new(opts).run_gen();
}
