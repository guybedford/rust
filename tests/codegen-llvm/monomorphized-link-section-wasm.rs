//! On wasm, each reified `#[link_section]` const monomorphization contributes a blob to the
//! named wasm custom section via `wasm.custom_sections` metadata.
//@ add-minicore
//@ compile-flags: -Copt-level=0 -Ccodegen-units=1 --target wasm32-unknown-unknown
//@ needs-llvm-components: webassembly

#![crate_type = "lib"]
#![no_std]
#![no_core]
#![feature(no_core, lang_items, generic_const_items, monomorphized_link_section)]
#![allow(incomplete_features)]

extern crate minicore;

#[used]
#[link_section = "__descriptors"]
const DESC<T>: [u8; 4] = [1, 2, 3, 4];

fn read<T>() -> [u8; 4] {
    DESC::<T>
}

#[no_mangle]
pub fn a() -> [u8; 4] {
    read::<u32>()
}

#[no_mangle]
pub fn b() -> [u8; 4] {
    read::<u64>()
}

// Retained via `#[used]` (appears earlier in the module than the metadata below).
// CHECK: @llvm.used

// The blobs land in a wasm custom section rather than linear-memory data.
// CHECK: !wasm.custom_sections = !{
// CHECK: !{!"__descriptors", !"\01\02\03\04"}
