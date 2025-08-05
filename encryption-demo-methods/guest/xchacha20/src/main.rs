#![no_std]
#![no_main]

use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::XChaCha20;
use risc0_zkvm::guest::env;
risc0_zkvm_guest::entry!(main);

pub fn main() {
    let key: [u8; 32] = env::read();
    let nonce: [u8; 24] = env::read();
    let len: u32 = env::read();
    let mut plaintext = vec![0u8; len as usize];
    env::read_slice(&mut plaintext).unwrap();

    let mut cipher = XChaCha20::new(&key.into(), &nonce.into());
    cipher.apply_keystream(&mut plaintext);

    env::commit_slice(&plaintext);
}

