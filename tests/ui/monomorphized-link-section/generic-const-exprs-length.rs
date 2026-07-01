//! The reified static's length may depend on the type parameters via `generic_const_exprs`.
//@ build-pass
//@ ignore-wasm32 custom sections work differently on wasm
#![feature(generic_const_items, generic_const_exprs, monomorphized_link_section)]
#![allow(incomplete_features)]

use std::mem::size_of;

#[used]
#[link_section = "__descriptors"]
const DESC<T>: [u8; size_of::<T>()] = [0u8; size_of::<T>()]
where
    [(); size_of::<T>()]:;

fn emit<T>() -> usize
where
    [(); size_of::<T>()]:,
{
    DESC::<T>.len()
}

fn main() {
    // Distinct lengths => distinct blobs.
    let _ = emit::<u32>();
    let _ = emit::<[u64; 4]>();
}
