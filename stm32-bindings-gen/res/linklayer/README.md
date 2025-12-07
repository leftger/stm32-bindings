# linklayer-bindings

Raw bindings for the STM32 WBA link-layer middleware. This crate is generated automatically by `stm32-bindings-gen` and is intended to be consumed by higher-level wrappers that provide safe abstractions for BLE or IEEE 802.15.4 stacks.

## Overview

The generator pulls in STM-provided headers and static libraries, then runs `bindgen` to emit Rust FFI shims. No additional logic lives here—consumers should treat every item as `unsafe` and wrap it before use.

## Layout

- `src/bindings/`: Raw Rust modules produced by `bindgen`.
- `src/lib/`: Static libraries copied from the STM32CubeWBA middleware tree, gated behind Cargo features.
- `build.rs`: Emits the appropriate `cargo:rustc-link-*` directives based on the selected features.

## Usage

1. Ensure `stm32-bindings-gen` has been run so this crate exists in `build/linklayer-bindings`.
2. Add a path dependency in your Cargo manifest pointing at that directory.
3. Opt into the desired static library variant by enabling the matching `lib_*` feature exposed by this crate.
4. Call the generated functions through `unsafe` code and wrap them in a higher-level API before exposing them to the rest of your application.

## Feature Flags

Each feature named `lib_<variant>` selects one of the prebuilt static archives shipped by ST. Enable exactly the libraries required by your firmware configuration; the build script will link them automatically.

## Regeneration

If the upstream ST middleware or the bindings configuration changes, rerun `cargo run -p stm32-bindings-gen`. The generator will overwrite this crate with the latest bindings, static libraries, and metadata.