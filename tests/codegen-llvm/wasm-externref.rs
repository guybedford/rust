//! Verify that the wasm `externref` lang type lowers to the LLVM target
//! extension type `target("wasm.externref")` in function signatures and stays
//! a direct SSA value.

//@ add-minicore
//@ compile-flags: -Copt-level=3 --target wasm32-unknown-unknown
//@ needs-llvm-components: webassembly
//@ min-llvm-version: 23

#![crate_type = "lib"]
#![no_std]
#![no_core]
#![feature(no_core, lang_items)]

extern crate minicore;

#[lang = "externref"]
#[non_exhaustive]
pub struct externref;

extern "C" {
    fn create_ref() -> externref;
    fn use_ref(v: externref);
}

// CHECK: define {{.*}}target("wasm.externref") @describe(target("wasm.externref") {{.*}}%v)
#[no_mangle]
pub extern "C" fn describe(v: externref) -> externref {
    v
}

// CHECK-LABEL: @roundtrip
#[no_mangle]
pub extern "C" fn roundtrip() {
    // CHECK: %[[V:.+]] = {{.*}}call {{.*}}target("wasm.externref") @create_ref()
    // CHECK: call void @use_ref(target("wasm.externref") {{.*}}%[[V]])
    unsafe { use_ref(create_ref()) }
}
