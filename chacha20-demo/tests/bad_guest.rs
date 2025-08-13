#[cfg(not(rust_analyzer))]
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

#[cfg(rust_analyzer)]
mod methods {
    pub const GUEST_ELF: &[u8] = &[];
    pub const GUEST_ID: [u32; 8] = [0; 8];
}
#[cfg(rust_analyzer)]
use methods::*;

use risc0_zkvm::{default_prover, ExecutorEnv};

#[test]
fn guest_panics_on_bad_key() {
    // This key triggers the panic in the guest
    let key = [0xFFu8; 32];
    let nonce = [0u8; 12];
    let plaintext = b"panic please".to_vec();

    let env = ExecutorEnv::builder()
        .write(&key).unwrap()
        .write(&nonce).unwrap()
        .write(&plaintext).unwrap()
        .build().unwrap();

    // Proving should fail when the guest panics
    let res = default_prover().prove(env, GUEST_ELF);
    assert!(res.is_err(), "proving should fail when guest panics");
}
