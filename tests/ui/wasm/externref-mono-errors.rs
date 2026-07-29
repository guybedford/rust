//! wasm `externref` values cannot be stored in memory. Most violating
//! positions are rejected at type-check time, but values captured into
//! coroutine/async state across a suspension point only become knowable at
//! monomorphization time and are rejected by the mono-check backstop.

//@ only-wasm32
//@ build-fail
//@ edition: 2021
//@ compile-flags: --crate-type=lib

#![feature(wasm_externref, coroutines, coroutine_trait, stmt_expr_attributes)]

use core::arch::wasm32::externref;
use core::future::Future;
use core::ops::Coroutine;

extern "C" {
    fn create_ref() -> externref;
    fn use_ref(v: externref);
}

async fn nop() {}

pub fn held_across_await() -> impl Future<Output = ()> {
    //~^ ERROR cannot exist: it contains a wasm `externref`
    async {
        let v = unsafe { create_ref() };
        nop().await;
        unsafe { use_ref(v) };
    }
}

pub fn held_across_yield() -> impl Coroutine<Yield = u32, Return = ()> {
    //~^ ERROR cannot exist: it contains a wasm `externref`
    #[coroutine]
    || {
        let v = unsafe { create_ref() };
        yield 1;
        unsafe { use_ref(v) };
    }
}
