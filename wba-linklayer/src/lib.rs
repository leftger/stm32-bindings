#![no_std]

/// High-level helpers around the STM32 WBA link layer primitives.
///
/// This crate consolidates the link-layer-specific pieces that are common to
/// both the BLE and IEEE 802.15.4 (MAC) stacks. It depends on the generated
/// raw bindings crate for the FFI definitions.
pub mod ffi;

pub mod linklayer_plat;

pub mod ll_sys_if;

pub mod ble_sys_if;

pub mod mac_sys_if;
