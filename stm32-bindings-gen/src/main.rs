use std::path::PathBuf;

use stm32_bindings_gen::{Gen, Options};

use clap::Parser;

/// A simple CLI tool
#[derive(Parser, Debug)]
#[command(
    name = "stm32-bindings-gen",
    version = env!("CARGO_PKG_VERSION"),
    about = "Generation of Bindings for STM32 Middlewares"
)]
struct Cli {
    /// Sources Directory
    #[arg(long, default_value = "sources")]
    sources_dir: PathBuf,

    /// Build Directory
    #[arg(long, default_value = "build")]
    build_dir: PathBuf,

    /// If given, only build this module
    #[arg(long, default_value = None)]
    module: Option<String>,
}

fn main() {
    let args = Cli::parse();

    let opts = Options {
        build_dir: args.build_dir,
        sources_dir: args.sources_dir,
    };

    Gen::new(opts).run_gen(&args.module);
}
