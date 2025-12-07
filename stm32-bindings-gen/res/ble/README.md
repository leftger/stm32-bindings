# ble-bindings

Raw bindings for the STM32 WBA Bluetooth Low Energy stack. This crate is generated automatically by `stm32-bindings-gen` and is meant to be consumed by higher-level wrappers that expose safe BLE abstractions.

## Overview

The generator ingests the ST middleware headers and prebuilt static libraries, then uses `bindgen` to emit Rust FFI items. Everything in this crate mirrors the original C API one-to-one; you should treat every item as `unsafe` and build ergonomic wrappers in a separate crate.

## Layout

- `src/bindings/`: Modules produced by `bindgen`, containing raw FFI definitions.
- `src/lib/`: Static libraries copied from the STM32CubeWBA distribution. Selecting the proper feature toggles which archives get linked.
- `build.rs`: Registers the static libraries with Cargo based on the enabled features.

## Usage

1. Run `cargo run -p stm32-bindings-gen` so this crate is regenerated under `build/ble-bindings`.
2. Add a path dependency in your Cargo manifest pointing to that directory.
3. Enable the feature corresponding to the static library variant required by your project (e.g. `lib_stm32wba_ble_stack_full`).
4. Wrap the raw FFI functions in a higher-level API before exposing them to the rest of your system.

## Feature Flags

Each `lib_*` feature matches a static archive provided by ST. Enable exactly the variants you need and the build script will emit the proper `cargo:rustc-link-lib` entries.

## Regeneration

If the underlying middleware or binding configuration changes, rerun `cargo run -p stm32-bindings-gen`. The generator will overwrite this crate with the latest bindings, libraries, and metadata.