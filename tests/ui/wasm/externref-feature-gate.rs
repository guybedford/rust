//! Using the wasm `externref` type requires `feature(wasm_externref)`.

//@ only-wasm32
//@ edition: 2021
//@ compile-flags: --crate-type=lib

use core::arch::wasm32::externref;
//~^ ERROR use of unstable library feature `wasm_externref`

extern "C" {
    fn use_ref(v: externref);
    //~^ ERROR use of unstable library feature `wasm_externref`
}
