use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;
use risc0_zkvm::guest::env;

pub fn main() {
    // Read 32-byte key
    let key: [u8; 32] = env::read();
    // Read 12-byte nonce
    let nonce: [u8; 12] = env::read();
    // Read plaintext length
    let len: u32 = env::read();
    // Read plaintext bytes
    let mut plaintext: Vec<_> = env::read();

    // Encrypt in-place
    let mut cipher = ChaCha20::new(&key.into(), &nonce.into());
    cipher.apply_keystream(&mut plaintext);

    // Commit ciphertext
    env::commit(&plaintext);
}

