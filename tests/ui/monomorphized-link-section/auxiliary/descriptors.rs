#![feature(generic_const_items, monomorphized_link_section)]
#![allow(incomplete_features)]

#[used]
#[link_section = "__descriptors"]
pub const DESC<T>: [u8; 4] = [1, 2, 3, 4];

pub fn emit<T>() -> u8 {
    DESC::<T>[0]
}
