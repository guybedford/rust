//! Each monomorphization of a generic `#[link_section]` const is reified into its own
//! `linkonce_odr` read-only global, placed in the named section and retained via `llvm.used`.
//@ ignore-wasm32 custom sections work differently on wasm
//@ compile-flags: -C no-prepopulate-passes -Copt-level=0

#![crate_type = "lib"]
#![feature(generic_const_items, monomorphized_link_section)]
#![allow(incomplete_features)]

#[used]
#[link_section = "__descriptors"]
const DESC<T>: [u8; 4] = [1, 2, 3, 4];

fn read<T>() -> u8 {
    DESC::<T>[0]
}

// Two distinct instantiations => two distinct globals, both in the section, both linkonce_odr.

// CHECK: @{{.*}}DESC{{.*}} = {{(linkonce_odr|weak_odr).*}}constant [4 x i8] c"\01\02\03\04", section "__descriptors"
// CHECK: @{{.*}}DESC{{.*}} = {{(linkonce_odr|weak_odr).*}}constant [4 x i8] c"\01\02\03\04", section "__descriptors"

// The reified statics are retained via `#[used]`.
// CHECK: @llvm.used = {{.*}}appending global

#[no_mangle]
pub fn a() -> u8 {
    read::<u32>()
}

#[no_mangle]
pub fn b() -> u8 {
    read::<u64>()
}
