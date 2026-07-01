//! A generic `#[link_section]` const defined upstream is reified (codegened) at the downstream
//! instantiation site, for both upstream- and downstream-chosen type parameters.
//@ run-pass
//@ ignore-wasm32 custom sections work differently on wasm
//@ aux-build: descriptors.rs

extern crate descriptors;

struct Local;

fn main() {
    // Instantiation with a foreign type, driven from the downstream crate.
    assert_eq!(descriptors::DESC::<u32>[0], 1);
    // Instantiation with a downstream-local type.
    assert_eq!(descriptors::DESC::<Local>[1], 2);
    assert_eq!(descriptors::emit::<u64>(), 1);
}
