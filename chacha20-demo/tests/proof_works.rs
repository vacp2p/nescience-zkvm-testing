#[cfg(not(rust_analyzer))]
include!{concat!(env!("OUT_DIR"), "/methods.rs")}

#[cfg(rust_analyzer)]
mod methods {
    pub const GUEST_ELF: &[u8] = &[];
    pub const GUEST_ID: [u32; 8] = [0; 8];
}
#[cfg(rust_analyzer)]
use methods::*;

use risc0_zkvm::{default_prover, ExecutorEnv};

// Host-side ChaCha20
use chacha20::{ChaCha20, Key, Nonce};
use cipher::{KeyIvInit, StreamCipher};

#[test]
fn proof_works_and_matches_host_chacha() {
    // Inputs (must match what your guest expects)
    let key       = [0x42u8; 32];
    let nonce     = [0x24u8; 12];
    let plaintext = b"Hello, RISC Zero ChaCha20 demo!";

    // Prove with the R0 guest
    let env = ExecutorEnv::builder()
        .write(&key).unwrap()
        .write(&nonce).unwrap()
        .write(&plaintext.to_vec()).unwrap()
        .build().unwrap();

    let prove_info = default_prover().prove(env, GUEST_ELF).expect("prove failed");
    prove_info.receipt.verify(GUEST_ID).expect("verify failed");

    
    // Ciphertext produced by the guest
    let guest_ct = prove_info.receipt.journal.bytes.clone();

    // Host-side reference: ChaCha20 with the same key/nonce
    let mut host_ct = plaintext.to_vec();
    let key = Key::from_slice(&key);
    let nonce = Nonce::from_slice(&nonce);
    let mut cipher = ChaCha20::new(key, nonce);
    cipher.apply_keystream(&mut host_ct);

    // Compare
    assert_eq!(guest_ct, host_ct, "guest ciphertext != host ChaCha20 ciphertext");

}
