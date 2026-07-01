//! End-to-end: a generic `#[link_section]`/`#[used]` const, referenced from a generic fn,
//! reifies one retained static per instantiation and links successfully.
//@ build-pass
//@ ignore-wasm32 custom sections work differently on wasm
#![feature(generic_const_items, monomorphized_link_section)]
#![allow(incomplete_features)]

#[used]
#[link_section = "__descriptors"]
const DESC<T>: [u8; 4] = [1, 2, 3, 4];

fn read<T>() -> u8 {
    DESC::<T>[0]
}

fn main() {
    // Two distinct instantiations, plus a repeat of the first to exercise dedup.
    assert_eq!(read::<u32>(), 1);
    assert_eq!(read::<u64>(), 1);
    assert_eq!(read::<u32>(), 1);
}
