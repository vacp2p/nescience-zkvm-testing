#![no_main]

use zkwasm_rust_sdk::wasm_dbg;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn zkmain() {
    for i in 0..100 {
        let hept = (5*i*i - 3*i)/2;
    }
}
