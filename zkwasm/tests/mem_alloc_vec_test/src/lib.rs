#![no_main]

use zkwasm_rust_sdk::wasm_dbg;
use wasm_bindgen::prelude::*;
use memory_allocations::{alloc_vec, push_vec, pop_vec};

#[wasm_bindgen]
pub fn zkmain() {
    let mut vvec = alloc_vec();

    push_vec(&mut vvec);

    pop_vec(&mut vvec);
}
