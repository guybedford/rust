//! Without `#![feature(monomorphized_link_section)]`, `#[used]` cannot be applied to a
//! generic `const` item.
// gate-test-monomorphized_link_section
#![feature(generic_const_items)]
#![allow(incomplete_features)]

#[used] //~ ERROR attribute cannot be used on
#[link_section = "__descriptors"] //~ WARN attribute cannot be used on
//~^ WARN this was previously accepted by the compiler but is being phased out
const DESC<T>: [u8; 4] = [1, 2, 3, 4];

fn main() {}
