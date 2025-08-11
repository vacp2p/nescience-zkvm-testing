#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;

use risc0_zkvm::guest::env;
risc0_zkvm::entry!(main);

use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};

fn main() {
    let key: [u8; 32]   = env::read();
    let nonce: [u8; 12] = env::read();
    let mut buf: Vec<u8> = env::read();

    let mut cipher = ChaCha20::new(&key.into(), &nonce.into());
    cipher.apply_keystream(&mut buf);

    env::commit_slice(&buf);
}
