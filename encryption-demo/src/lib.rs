//! Host-side helpers: prove encryption inside RISC Zero and verify receipts.

use risc0_zkvm::{
    default_prover, ExecutorEnv, Receipt,
};
use encryption_demo_methods::{
    CHACHA20_ELF, CHACHA20_ID, XCHACHA20_ELF, XCHACHA20_ID,
};

/// Encrypt `plaintext` with ChaCha20 inside the zkVM.
/// Returns `(ciphertext, receipt)`.
pub fn encrypt_chacha20(
    key: &[u8; 32],
    nonce: &[u8; 12],
    plaintext: &[u8],
) -> anyhow::Result<(Vec<u8>, Receipt)> {
    let env = ExecutorEnv::builder()
        .write(key)?
        .write(nonce)?
        .write(&(plaintext.len() as u32))?
        .write_slice(plaintext)?
        .build()?;

    let prover = default_prover();
    let receipt = prover.prove(env, CHACHA20_ELF)?;
    receipt.verify(CHACHA20_ID)?;

    let ciphertext: Vec<u8> = receipt.journal.decode()?;
    Ok((ciphertext, receipt))
}

/// Same API for XChaCha20 (24-byte nonce)
pub fn encrypt_xchacha20(
    key: &[u8; 32],
    nonce: &[u8; 24],
    plaintext: &[u8],
) -> anyhow::Result<(Vec<u8>, Receipt)> {
    let env = ExecutorEnv::builder()
        .write(key)?
        .write(nonce)?
        .write(&(plaintext.len() as u32))?
        .write_slice(plaintext)?
        .build()?;

    let prover = default_prover();
    let receipt = prover.prove(env, XCHACHA20_ELF)?;
    receipt.verify(XCHACHA20_ID)?;

    let ciphertext: Vec<u8> = receipt.journal.decode()?;
    Ok((ciphertext, receipt))
}


// Test for chacha20
#[cfg(test)]
mod tests {
    use super::*;
    use chacha20::cipher::{KeyIvInit, StreamCipher};
    use chacha20::ChaCha20;

    // RFC 8439 test vector
    const KEY: [u8; 32] = [
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00
    ];
    const NONCE: [u8; 12] = [0; 12];
    const PLAINTEXT: [u8; 16] = *b"example message!";

    #[test]
    fn chacha20_vector_matches() {
        let (ciphertext, _) = encrypt_chacha20(&KEY, &NONCE, &PLAINTEXT).unwrap();

        // host decryption
        let mut buf = ciphertext.clone();
        let mut cipher = ChaCha20::new(&KEY.into(), &NONCE.into());
        cipher.apply_keystream(&mut buf);

        assert_eq!(&buf, &PLAINTEXT);
    }
}


// Tests for Xchacha20
#[cfg(test)]
mod tests {
    use super::*;
    use xchacha20::cipher::{KeyIvInit, StreamCipher};
    use xchacha20::xChaCha20;

    // RFC 8439 test vector
    const KEY: [u8; 32] = [
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00
    ];
    const NONCE: [u8; 12] = [0; 12];
    const PLAINTEXT: [u8; 16] = *b"example message!";

    #[test]
    fn xchacha20_vector_matches() {
        let (ciphertext, _) = encrypt_xchacha20(&KEY, &NONCE, &PLAINTEXT).unwrap();

        // host decryption
        let mut buf = ciphertext.clone();
        let mut cipher = xChaCha20::new(&KEY.into(), &NONCE.into());
        cipher.apply_keystream(&mut buf);

        assert_eq!(&buf, &PLAINTEXT);
    }
}

