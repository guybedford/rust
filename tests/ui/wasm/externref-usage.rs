//! Legal wasm `externref` usage with the real `core` type: bare parameter,
//! return and local slots (including mutable rebinding and function pointer
//! signature slots), and async bodies where no value is live across an await.

//@ only-wasm32
//@ build-pass
//@ min-llvm-version: 23
//@ edition: 2021
//@ compile-flags: --crate-type=lib

#![feature(wasm_externref)]
#![allow(unused_assignments)]

use core::arch::wasm32::externref;
use core::future::Future;

extern "C" {
    fn create_ref() -> externref;
    fn use_ref(v: externref);
}

#[unsafe(no_mangle)]
pub extern "C" fn roundtrip(v: externref) -> externref {
    v
}

pub fn locals() {
    let v = unsafe { create_ref() };
    let w = v; // moves like any non-Copy value
    unsafe { use_ref(w) };
}

pub fn mutable_local() {
    unsafe {
        let mut v = create_ref();
        v = create_ref();
        use_ref(v);
    }
}

pub fn fn_ptr_slots(v: externref) -> externref {
    let f: extern "C" fn(externref) -> externref = roundtrip;
    f(v)
}

async fn nop() {}

// externref locals that are dead before every suspension point never enter
// the coroutine state and are sound.
pub fn dead_before_await() -> impl Future<Output = ()> {
    async {
        let v = unsafe { create_ref() };
        unsafe { use_ref(v) };
        nop().await;
    }
}
